//! Product Analytics：獨立持久冪等事件接收端。
//! 事件內容嚴格四欄，不存聊天內文、房間 ID 或使用者識別。
use crate::chat::ProductEvent;
use axum::{Json, Router, extract::State, http::{HeaderMap, StatusCode}, routing::post};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct IngestState { pub pool: PgPool, pub token: Arc<str> }

pub async fn ingest(
    State(state): State<IngestState>,
    headers: HeaderMap,
    Json(event): Json<ProductEvent>,
) -> StatusCode {
    // 不使用固定值或空 token；認證失敗不能寫入任何事件。
    if state.token.is_empty() { return StatusCode::SERVICE_UNAVAILABLE; }
    let expected = format!("Bearer {}", state.token);
    if headers.get("authorization").and_then(|h| h.to_str().ok()) != Some(expected.as_str()) {
        return StatusCode::UNAUTHORIZED;
    }
    if event.schema_version != 1
        || !matches!(event.source_app.as_str(), "web_chat" | "desktop_pet")
        || event.event_type.is_empty() || event.event_type.len()>128 {
        return StatusCode::BAD_REQUEST;
    }
    // (source_app,event_id) 資料庫唯一約束，重啟/競爭後仍冪等。
    match sqlx::query(
        "INSERT INTO product_events (source_app, event_id, schema_version, event_type)
         VALUES ($1,$2,$3,$4) ON CONFLICT (source_app,event_id) DO NOTHING"
    ).bind(&event.source_app).bind(event.event_id)
      .bind(i32::from(event.schema_version)).bind(&event.event_type)
      .execute(&state.pool).await {
        Ok(result) if result.rows_affected() == 1 => StatusCode::CREATED,
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

pub fn router(pool: PgPool, token: impl Into<Arc<str>>) -> Router {
    Router::new().route("/api/v1/events", post(ingest))
      .with_state(IngestState { pool, token: token.into() })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strict_event_schema_rejects_extra_fields() {
        let json = r#"{"schema_version":1,"event_id":"00000000-0000-0000-0000-000000000001","source_app":"web_chat","event_type":"message.persisted","content":"PRIVATE"}"#;
        assert!(serde_json::from_str::<ProductEvent>(json).is_err());
    }
    #[test]
    fn missing_uuid_is_not_accepted() {
        let json = r#"{"schema_version":1,"event_id":"bad","source_app":"web_chat","event_type":"message.persisted"}"#;
        assert!(serde_json::from_str::<ProductEvent>(json).is_err());
    }
}
