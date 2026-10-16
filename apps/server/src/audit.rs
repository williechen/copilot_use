//! 管理者專用稽核；登入與建房的同筆交易記錄，對外不公開使用者身份。
use super::{StateData, authorized};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use event_contract::audit::{AuditQuery, AuditRecord};
use uuid::Uuid;
pub(super) struct Entry<'a> {
    pub id: Uuid,
    pub kind: &'a str,
    pub user_id: Option<Uuid>,
    pub username: &'a str,
    pub room_id: Option<&'a str>,
    pub room_name: Option<&'a str>,
    pub target_username: Option<&'a str>,
}
pub(super) async fn record(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    entry: Entry<'_>,
) -> Result<(), StatusCode> {
    sqlx::query("INSERT INTO chat_activity_audit(event_id,event_type,user_id,username,room_id,room_name,target_username) VALUES(?1,?2,?3,?4,?5,?6,?7)")
      .bind(entry.id).bind(entry.kind).bind(entry.user_id).bind(entry.username).bind(entry.room_id).bind(entry.room_name).bind(entry.target_username)
      .execute(&mut **tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(())
}
pub(super) async fn query(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<AuditQuery>,
) -> Result<Json<Vec<AuditRecord>>, StatusCode> {
    if !authorized(&headers, &state.audit_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if !input.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows=sqlx::query_as::<_,(Uuid,String,Option<Uuid>,String,Option<String>,Option<String>,Option<String>,i64)>("SELECT event_id,event_type,user_id,username,room_id,room_name,target_username,occurred_unix_ms FROM chat_activity_audit WHERE (?1 IS NULL OR username=?1) AND (?2 IS NULL OR event_type=?2) AND (?3 IS NULL OR (event_type!='login.failed')=?3) AND (?4 IS NULL OR occurred_unix_ms>=?4) AND (?5 IS NULL OR occurred_unix_ms<?5) ORDER BY occurred_unix_ms DESC,event_id DESC LIMIT 100")
       .bind(input.username).bind(input.event_type).bind(input.succeeded).bind(input.from_unix_ms).bind(input.to_unix_ms)
       .fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(
        rows.into_iter()
            .map(
                |(
                    event_id,
                    event_type,
                    user_id,
                    username,
                    room_id,
                    room_name,
                    target_username,
                    occurred_unix_ms,
                )| AuditRecord {
                    succeeded: event_type != "login.failed",
                    event_id,
                    event_type,
                    user_id,
                    username,
                    room_id,
                    room_name,
                    target_username,
                    occurred_unix_ms,
                },
            )
            .collect(),
    ))
}
#[cfg(test)]
mod tests;
/// 個人管理區只可查看自己的操作紀錄；忽略輸入的其他帳號名稱。
pub(super) async fn mine(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<AuditQuery>,
) -> Result<Json<Vec<AuditRecord>>, StatusCode> {
    let user = super::auth::principal(&state, &headers, false)
        .await?
        .user_id()
        .ok_or(StatusCode::FORBIDDEN)?;
    if !input.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows=sqlx::query_as::<_,(Uuid,String,String,Option<String>,Option<String>,Option<String>,i64)>("SELECT event_id,event_type,username,room_id,room_name,target_username,occurred_unix_ms FROM chat_activity_audit WHERE user_id=?1 AND (?2 IS NULL OR event_type=?2) AND (?3 IS NULL OR (event_type!='login.failed')=?3) AND (?4 IS NULL OR occurred_unix_ms>=?4) AND (?5 IS NULL OR occurred_unix_ms<?5) ORDER BY occurred_unix_ms DESC,event_id DESC LIMIT 100").bind(user).bind(input.event_type).bind(input.succeeded).bind(input.from_unix_ms).bind(input.to_unix_ms).fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(
        rows.into_iter()
            .map(
                |(
                    event_id,
                    event_type,
                    username,
                    room_id,
                    room_name,
                    target_username,
                    occurred_unix_ms,
                )| AuditRecord {
                    succeeded: event_type != "login.failed",
                    event_id,
                    event_type,
                    user_id: Some(user),
                    username,
                    room_id,
                    room_name,
                    target_username,
                    occurred_unix_ms,
                },
            )
            .collect(),
    ))
}
