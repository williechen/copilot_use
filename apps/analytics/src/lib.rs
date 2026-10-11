//! Analytics：認證後持久事件、依 (source_app,event_id) 原子去重、Query API。
pub mod dashboard;
mod governance;
mod insights;
use axum::{
    Router,
    extract::{Json, Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use event_contract::ProductEvent;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::{PgPool, Row};
use std::sync::Arc;

#[derive(Clone)]
pub struct AnalyticsState {
    pub pool: PgPool,
    pub api_token: Arc<str>,
    pub chat_token: Arc<str>,
    pub pet_token: Arc<str>,
    pub admin_token: Arc<str>,
    pub ai_slots: Arc<tokio::sync::Semaphore>,
}

#[derive(Clone)]
pub struct EventStore {
    pool: PgPool,
}

impl EventStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 真正持久化成功才回 2xx；同一來源/事件的重送不再增加計數。
    pub async fn insert_once(&self, event: &ProductEvent) -> Result<bool, sqlx::Error> {
        use sha2::{Digest, Sha256};
        let key =
            Sha256::digest(format!("{}:{}", event.source_app, event.event_id).as_bytes()).to_vec();
        let kind = Sha256::digest(event.event_type.as_bytes()).to_vec();
        let mut tx = self.pool.begin().await?;
        let inserted=sqlx::query("INSERT INTO event_dedup_keys(key_hash,type_hash) VALUES ($1,$2) ON CONFLICT DO NOTHING").bind(&key).bind(&kind).execute(&mut *tx).await?;
        if inserted.rows_affected() == 0 {
            let previous: Vec<u8> =
                sqlx::query_scalar("SELECT type_hash FROM event_dedup_keys WHERE key_hash=$1")
                    .bind(key)
                    .fetch_one(&mut *tx)
                    .await?;
            if previous != kind {
                return Err(sqlx::Error::Protocol("event_identity_conflict".into()));
            }
            tx.commit().await?;
            return Ok(false);
        }
        sqlx::query("INSERT INTO product_events(source_app,event_id,schema_version,event_type) VALUES ($1,$2,$3,$4)")
            .bind(&event.source_app).bind(event.event_id).bind(i16::try_from(event.schema_version).unwrap_or(-1)).bind(&event.event_type).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(true)
    }

    pub async fn summary(&self) -> Result<Vec<EventSummary>, sqlx::Error> {
        sqlx::query_as::<_, EventSummary>(
            "SELECT source_app, event_type, count(*)::bigint AS total
             FROM product_events WHERE NOT archived
             GROUP BY source_app, event_type ORDER BY source_app, event_type",
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn total(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT count(*) AS total FROM product_events WHERE NOT archived")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get("total"))
    }
}

fn authorized(headers: &HeaderMap, expected: &str) -> bool {
    // 僅演示受控 Bearer token；實際部署應用 secret manager、TLS 與 token rotation。
    !expected.is_empty()
        && headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .is_some_and(|v| v == expected)
}

async fn ingest(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Json(event): Json<ProductEvent>,
) -> StatusCode {
    let expected = match event.source_app.as_str() {
        "web_chat" => &state.chat_token,
        "desktop_pet" => &state.pet_token,
        _ => return StatusCode::BAD_REQUEST,
    };
    if !authorized(&headers, expected) {
        return StatusCode::UNAUTHORIZED;
    }
    if !event.is_valid() {
        return StatusCode::BAD_REQUEST;
    }
    match EventStore::new(state.pool).insert_once(&event).await {
        Ok(_) => StatusCode::ACCEPTED,
        Err(sqlx::Error::Protocol(message)) if message == "event_identity_conflict" => {
            StatusCode::CONFLICT
        }
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

#[derive(Debug, Serialize, FromRow, PartialEq, Eq)]
pub struct EventSummary {
    pub source_app: String,
    pub event_type: String,
    pub total: i64,
}

#[derive(Serialize)]
struct CountResponse {
    total: i64,
}

async fn count(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
) -> Result<Json<CountResponse>, StatusCode> {
    if !authorized(&headers, &state.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let total = EventStore::new(state.pool)
        .total()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(CountResponse { total }))
}

async fn summary(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
) -> Result<Json<Vec<EventSummary>>, StatusCode> {
    if !authorized(&headers, &state.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let values = EventStore::new(state.pool)
        .summary()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(values))
}

async fn health(State(state): State<AnalyticsState>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

async fn event_lookup(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Path((source, id)): Path<(String, uuid::Uuid)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !authorized(&headers, &state.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let kind: Option<String> = sqlx::query_scalar(
        "SELECT event_type FROM product_events WHERE source_app=$1 AND event_id=$2 AND NOT archived",
    )
    .bind(source)
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    match kind {
        Some(kind) => Ok(Json(serde_json::json!({"event_id":id,"event_type":kind}))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub fn build_app(state: AnalyticsState) -> Router {
    Router::new()
        .route("/", get(dashboard::dashboard))
        .route("/healthz", get(health))
        .route("/api/v1/insights", post(insights::generate))
        .route("/api/v1/suggestions", get(insights::list))
        .route("/api/v1/suggestions/{id}", get(insights::get))
        .route("/api/v1/suggestions/{id}/review", post(insights::review))
        .route("/api/v1/suggestions/{id}/receipt", post(insights::receipt))
        .route(
            "/api/v1/suggestions/{id}/evaluate",
            post(insights::evaluate),
        )
        .route("/api/v1/events", post(ingest))
        .route("/api/v1/events/{source}/{id}", get(event_lookup))
        .route("/api/v1/events/count", get(count))
        .route("/api/v1/events/summary", get(summary))
        .route("/api/v1/events/timeline", get(governance::timeline))
        .route("/api/v1/events/archive", post(governance::archive))
        .route("/api/v1/events/restore", post(governance::restore))
        .layer(axum::extract::DefaultBodyLimit::max(4096))
        .layer(axum::middleware::from_fn(copilot_telemetry::http))
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
    async fn concurrent_delivery_of_same_event_is_idempotent(pool: PgPool) {
        let store = EventStore::new(pool);
        let event = ProductEvent::chat_message_persisted(Uuid::new_v4());
        let (first, second) = tokio::join!(store.insert_once(&event), store.insert_once(&event));
        assert!(first.unwrap() ^ second.unwrap(), "並發重送只有一筆成功新增");
        assert_eq!(store.total().await.unwrap(), 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn summary_counts_per_source_and_event_type(pool: PgPool) {
        let store = EventStore::new(pool);
        assert!(
            store
                .insert_once(&ProductEvent::chat_message_persisted(Uuid::new_v4()))
                .await
                .unwrap()
        );
        assert!(
            store
                .insert_once(&ProductEvent::chat_message_persisted(Uuid::new_v4()))
                .await
                .unwrap()
        );
        assert!(
            store
                .insert_once(&ProductEvent::pet_cleanup_succeeded(Uuid::new_v4()))
                .await
                .unwrap()
        );
        let values = store.summary().await.unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values.iter().map(|s| s.total).sum::<i64>(), 3);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn two_sources_same_event_id_count_separately(pool: PgPool) {
        let id = Uuid::new_v4();
        let store = EventStore::new(pool);
        assert!(
            store
                .insert_once(&ProductEvent::chat_message_persisted(id))
                .await
                .unwrap()
        );
        assert!(
            store
                .insert_once(&ProductEvent::pet_cleanup_succeeded(id))
                .await
                .unwrap()
        );
        assert_eq!(store.total().await.unwrap(), 2);
    }
    #[sqlx::test(migrations = "./migrations")]
    async fn archived_event_is_not_resurrected_by_retry_and_can_be_restored(pool: PgPool) {
        let store = EventStore::new(pool.clone());
        let event = ProductEvent::chat_message_persisted(Uuid::new_v4());
        assert!(store.insert_once(&event).await.unwrap());
        sqlx::query("UPDATE product_events SET archived=true")
            .execute(&pool)
            .await
            .unwrap();
        assert!(!store.insert_once(&event).await.unwrap());
        assert_eq!(store.total().await.unwrap(), 0);
        sqlx::query("UPDATE product_events SET archived=false")
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(store.total().await.unwrap(), 1);
    }
    #[sqlx::test(migrations = "./migrations")]
    async fn conflicting_retry_is_rejected(pool: PgPool) {
        let store = EventStore::new(pool);
        let mut event = ProductEvent::pet_cleanup_succeeded(Uuid::new_v4());
        assert!(store.insert_once(&event).await.unwrap());
        event.event_type = "question.answered".into();
        assert!(matches!(
            store.insert_once(&event).await,
            Err(sqlx::Error::Protocol(_))
        ));
        assert_eq!(store.total().await.unwrap(), 1);
    }
}
