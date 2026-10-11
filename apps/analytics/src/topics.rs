//! Source-authenticated withdrawal remains effective across late delivery and admin restore.
use super::{AnalyticsState, authorized};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use uuid::Uuid;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Withdrawal {
    event_ids: Vec<Uuid>,
}
pub(super) async fn withdraw(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Json(input): Json<Withdrawal>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !authorized(&headers, &state.chat_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if input.event_ids.is_empty()
        || input.event_ids.len() > 500
        || input.event_ids.iter().any(Uuid::is_nil)
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    withdraw_ids(&state.pool, &input.event_ids)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(serde_json::json!({"withdrawn":true})))
}
pub(crate) async fn withdraw_ids(pool: &sqlx::PgPool, ids: &[Uuid]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT INTO topic_privacy_state(event_id,withdrawn)SELECT event_id,true FROM(SELECT DISTINCT UNNEST($1::uuid[]) AS event_id) AS ids ORDER BY event_id ON CONFLICT(event_id)DO UPDATE SET withdrawn=true",
    )
    .bind(ids)
    .execute(&mut *tx)
    .await?;
    sqlx::query("UPDATE product_events SET consent_withdrawn=true,archived=true WHERE source_app='web_chat' AND event_type LIKE 'keyword.%' AND event_id::uuid=ANY($1)").bind(ids).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventStore;
    #[sqlx::test(migrations = "./migrations")]
    async fn withdrawal_before_delivery_and_retry_stays_excluded_after_admin_restore(
        pool: sqlx::PgPool,
    ) {
        let store = EventStore::new(pool.clone());
        let event = event_contract::ProductEvent {
            schema_version: 1,
            event_id: Uuid::new_v4(),
            source_app: "web_chat".into(),
            event_type: "keyword.rust".into(),
        };
        withdraw_ids(&pool, &[event.event_id]).await.unwrap();
        assert!(store.insert_once(&event).await.unwrap());
        assert_eq!(store.total().await.unwrap(), 0);
        sqlx::query("UPDATE product_events SET archived=false")
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(store.total().await.unwrap(), 0);
        assert!(!store.insert_once(&event).await.unwrap());
        assert!(
            store
                .lookup("web_chat", event.event_id)
                .await
                .unwrap()
                .is_none()
        );
        let message = event_contract::ProductEvent::chat_message_persisted(Uuid::new_v4());
        store.insert_once(&message).await.unwrap();
        withdraw_ids(&pool, &[message.event_id]).await.unwrap();
        assert_eq!(store.total().await.unwrap(), 1);
    }
}
