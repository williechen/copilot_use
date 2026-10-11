//! Query/Archive Adapter：Taipei 日界線、管理者確認與可還原封存。
use super::{AnalyticsState, authorized};
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Deserialize)]
pub(super) struct Range {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}
#[derive(Serialize, FromRow)]
pub(super) struct Daily {
    day: String,
    source_app: String,
    event_type: String,
    total: i64,
}
pub(super) async fn timeline(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Query(range): Query<Range>,
) -> Result<Json<Vec<Daily>>, StatusCode> {
    if !authorized(&headers, &state.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if range.to <= range.from || (range.to - range.from).num_days() > 366 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows=sqlx::query_as("SELECT (received_at AT TIME ZONE 'Asia/Taipei')::date::text AS day,source_app,event_type,count(*)::bigint AS total FROM product_events WHERE NOT archived AND received_at >= $1::timestamptz AND received_at < $2::timestamptz GROUP BY day,source_app,event_type ORDER BY day,source_app,event_type")
  .bind(range.from.to_rfc3339()).bind(range.to.to_rfc3339()).fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(rows))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Archive {
    before: DateTime<Utc>,
    confirmed: bool,
}
async fn set_archive(
    state: AnalyticsState,
    headers: HeaderMap,
    input: Archive,
    archived: bool,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !authorized(&headers, &state.admin_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if !input.confirmed || input.before > Utc::now() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let result=sqlx::query("UPDATE product_events SET archived=$1 WHERE received_at < $2::timestamptz AND archived<>$1").bind(archived).bind(input.before.to_rfc3339()).execute(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(
        serde_json::json!({"changed":result.rows_affected(),"archived":archived,"reversible":true}),
    ))
}
pub(super) async fn archive(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Json(input): Json<Archive>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    set_archive(state, headers, input, true).await
}
pub(super) async fn restore(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Json(input): Json<Archive>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    set_archive(state, headers, input, false).await
}
