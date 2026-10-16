//! 各人的管理區：只列出自己建立的房間與成員，Session 身分由伺服器判定。
use super::{StateData, auth};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde_json::{Value, json};
use uuid::Uuid;
pub(super) async fn rooms(
    State(state): State<StateData>,
    headers: HeaderMap,
) -> Result<Json<Vec<Value>>, StatusCode> {
    let user = auth::principal(&state, &headers, false)
        .await?
        .user_id()
        .ok_or(StatusCode::FORBIDDEN)?;
    let rows = sqlx::query_as::<_, (String, String, bool)>(
        "SELECT id,name,is_private FROM chat_rooms WHERE owner_id=?1 ORDER BY name,id LIMIT 100",
    )
    .bind(user)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(rows.into_iter().map(|(id,name,is_private)|json!({"id":id,"name":name,"is_private":is_private,"can_manage":true})).collect()))
}
pub(super) async fn members(
    State(state): State<StateData>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    let user = auth::principal(&state, &headers, false)
        .await?
        .user_id()
        .ok_or(StatusCode::FORBIDDEN)?;
    let owner: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM chat_rooms WHERE id=?1 AND owner_id=?2)")
            .bind(&id)
            .bind(user)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    if !owner {
        return Err(StatusCode::FORBIDDEN);
    }
    let rows=sqlx::query_as::<_,(Uuid,String)>("SELECT u.id,u.username FROM chat_room_members m JOIN chat_users u ON u.id=m.user_id WHERE m.room_id=?1 ORDER BY u.username,u.id LIMIT 100").bind(id).fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(
        rows.into_iter()
            .map(|(id, username)| json!({"id":id,"username":username,"owner":id==user}))
            .collect(),
    ))
}
pub(super) async fn join(
    State(state): State<StateData>,
    headers: HeaderMap,
    Path(room): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let user = auth::principal(&state, &headers, true)
        .await?
        .user_id()
        .ok_or(StatusCode::FORBIDDEN)?;
    let mut tx = state
        .pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let allowed:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM chat_rooms r WHERE r.id=?1 AND (NOT r.is_private OR EXISTS(SELECT 1 FROM chat_room_members m WHERE m.room_id=r.id AND m.user_id=?2)))").bind(&room).bind(user).fetch_one(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    if !allowed {
        return Err(StatusCode::FORBIDDEN);
    }
    sqlx::query(
        "INSERT INTO chat_room_members(room_id,user_id)VALUES(?1,?2)ON CONFLICT DO NOTHING",
    )
    .bind(room)
    .bind(user)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(StatusCode::NO_CONTENT)
}
