//! Analytics：認證後持久事件、依 (source_app,event_id) 原子去重、Query API。
use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Router,
};
use event_contract::ProductEvent;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;

#[derive(Clone)]
pub struct AnalyticsState {
    pub pool: PgPool,
    pub api_token: Arc<str>,
}

#[derive(Clone)]
pub struct EventStore {
    pool: PgPool,
}

impl EventStore {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    /// 真正持久化成功才回 2xx；同一來源/事件的重送不再增加計數。
    pub async fn insert_once(&self, event: &ProductEvent) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO product_events (source_app,event_id,schema_version,event_type)
             VALUES ($1,$2,$3,$4) ON CONFLICT (source_app,event_id) DO NOTHING"
        )
        .bind(&event.source_app).bind(event.event_id)
        .bind(i16::try_from(event.schema_version).unwrap_or(-1))
        .bind(&event.event_type)
        .execute(&self.pool).await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn total(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT count(*) AS total FROM product_events")
            .fetch_one(&self.pool).await?;
        Ok(row.get("total"))
    }
}

fn authorized(headers: &HeaderMap, expected: &str) -> bool {
    // 僅演示受控 Bearer token；實際部署應用 secret manager、TLS 與 token rotation。
    !expected.is_empty()
        && headers.get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .is_some_and(|v| v == expected)
}

async fn ingest(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Json(event): Json<ProductEvent>,
) -> StatusCode {
    if !authorized(&headers, &state.api_token) { return StatusCode::UNAUTHORIZED; }
    if !event.is_valid() { return StatusCode::BAD_REQUEST; }
    match EventStore::new(state.pool).insert_once(&event).await {
        Ok(_) => StatusCode::ACCEPTED,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

#[derive(Serialize)]
struct CountResponse { total: i64 }

async fn count(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
) -> Result<Json<CountResponse>, StatusCode> {
    if !authorized(&headers, &state.api_token) { return Err(StatusCode::UNAUTHORIZED); }
    let total = EventStore::new(state.pool).total().await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(CountResponse { total }))
}

async fn health() -> StatusCode { StatusCode::OK }

pub fn build_app(state: AnalyticsState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/api/v1/events", post(ingest))
        .route("/api/v1/events/count", get(count))
        .layer(axum::extract::DefaultBodyLimit::max(4096))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn missing_auth_rejected() {
        assert!(!authorized(&HeaderMap::new(), "test-token"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn same_source_event_counts_once_even_after_new_store(pool: PgPool) {
        let event = ProductEvent::chat_message_persisted(Uuid::new_v4());
        let a = EventStore::new(pool.clone());
        assert!(a.insert_once(&event).await.unwrap());
        let b = EventStore::new(pool.clone());
        assert!(!b.insert_once(&event).await.unwrap());
        assert_eq!(b.total().await.unwrap(), 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn two_sources_same_event_id_count_separately(pool: PgPool) {
        let id = Uuid::new_v4();
        let store = EventStore::new(pool);
        assert!(store.insert_once(&ProductEvent::chat_message_persisted(id)).await.unwrap());
        assert!(store.insert_once(&ProductEvent::pet_cleanup_succeeded(id)).await.unwrap());
        assert_eq!(store.total().await.unwrap(), 2);
    }
}
