//! Authenticated SSE invalidation, derived from committed PostgreSQL state across instances.
use crate::{AnalyticsState, authorized};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::sse::{Event, KeepAlive, Sse},
};
use std::{
    convert::Infallible,
    sync::{Arc, LazyLock},
    time::Duration,
};
static SLOTS: LazyLock<Arc<tokio::sync::Semaphore>> =
    LazyLock::new(|| Arc::new(tokio::sync::Semaphore::new(16)));
async fn revision(pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT revision FROM analytics_revision WHERE singleton")
        .fetch_one(pool)
        .await
}
pub async fn updates(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
) -> Result<Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    if !authorized(&headers, &state.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let permit = SLOTS
        .clone()
        .try_acquire_owned()
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;
    let mut current = revision(&state.pool)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let stream = async_stream::stream! {
        let _permit = permit;
        // Every connection gets a snapshot notification; reconnect never relies on volatile history.
        yield Ok(Event::default().event("invalidate").id(current.to_string()).data(current.to_string()).retry(Duration::from_secs(2)));
        let mut ticks = tokio::time::interval(Duration::from_secs(1));
        ticks.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            ticks.tick().await;
            match revision(&state.pool).await {
                Ok(next) if next != current => {
                    current = next;
                    yield Ok(Event::default().event("invalidate").id(current.to_string()).data(current.to_string()));
                }
                Ok(_) => {},
                // Close stream on DB failure; reconnect must pass readiness again.
                Err(_) => break,
            }
        }
    };
    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive"),
    ))
}
