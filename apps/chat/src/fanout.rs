//! Cross-process notifications carry only message IDs; canonical content is fetched after commit.
use super::{StateData, auth, realtime::RoomEvent};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde_json::{Value, json};
use sqlx::{Row, SqlitePool};
use tokio::sync::broadcast;
use uuid::Uuid;
pub(super) async fn start(
    _url: &str,
    pool: SqlitePool,
    events: broadcast::Sender<RoomEvent>,
) -> Result<tokio::task::JoinHandle<()>, sqlx::Error> {
    let mut cursor: i64 = sqlx::query_scalar("SELECT COALESCE(max(seq),0) FROM chat_realtime")
        .fetch_one(&pool)
        .await?;
    Ok(tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));
        let mut ticks = 0_u32;
        loop {
            interval.tick().await;
            let rows = sqlx::query(
                "SELECT seq,payload FROM chat_realtime WHERE seq>?1 ORDER BY seq LIMIT 256",
            )
            .bind(cursor)
            .fetch_all(&pool)
            .await;
            let Ok(rows) = rows else {
                let _ = events.send(RoomEvent {
                    room_id: "*".into(),
                    payload: json!({"type":"resync_required"}),
                });
                continue;
            };
            for row in rows {
                let seq: i64 = row.get("seq");
                if seq > cursor + 1 {
                    let _ = events.send(RoomEvent {
                        room_id: "*".into(),
                        payload: json!({"type":"resync_required"}),
                    });
                }
                cursor = seq;
                let Ok(mut value) = serde_json::from_str::<Value>(row.get("payload")) else {
                    continue;
                };
                let Some(room) = value["room_id"]
                    .as_str()
                    .filter(|s| !s.is_empty() && s.len() <= 40)
                    .map(str::to_owned)
                else {
                    continue;
                };
                if value["type"] == "message" {
                    let Some(id) = value["id"].as_str().and_then(|s| Uuid::parse_str(s).ok())
                    else {
                        continue;
                    };
                    let content: Result<Option<String>, _> = sqlx::query_scalar(
                        "SELECT content FROM chat_messages WHERE id=?1 AND room_id=?2",
                    )
                    .bind(id)
                    .bind(&room)
                    .fetch_optional(&pool)
                    .await;
                    value = match content {
                        Ok(Some(content)) => json!({"type":"message","id":id,"content":content}),
                        _ => json!({"type":"resync_required"}),
                    };
                } else if matches!(value["type"].as_str(), Some("presence" | "typing"))
                    && value["connection_id"]
                        .as_str()
                        .and_then(|s| Uuid::parse_str(s).ok())
                        .is_some()
                {
                    value.as_object_mut().unwrap().remove("room_id");
                } else {
                    continue;
                }
                let _ = events.send(RoomEvent {
                    room_id: room,
                    payload: value,
                });
            }
            ticks += 1;
            if ticks.is_multiple_of(1200) {
                let _ = sqlx::query(
                    "DELETE FROM chat_realtime WHERE created_at<unixepoch('subsec')-60 AND seq<=?1",
                )
                .bind(cursor)
                .execute(&pool)
                .await;
            }
        }
    }))
}
pub(super) async fn publish(
    pool: &SqlitePool,
    room: &str,
    mut payload: Value,
) -> Result<(), sqlx::Error> {
    payload["room_id"] = json!(room);
    sqlx::query("INSERT INTO chat_realtime(payload)VALUES(?1)")
        .bind(payload.to_string())
        .execute(pool)
        .await?;
    Ok(())
}
pub(super) async fn join(
    pool: &SqlitePool,
    room: &str,
    id: Uuid,
    user: Option<Uuid>,
) -> Result<(), StatusCode> {
    let mut tx = pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("SELECT 1")
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("DELETE FROM chat_connections WHERE expires_at<=unixepoch('subsec')")
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let (total, own): (i64, i64) =
        sqlx::query_as("SELECT count(*),count(*)FILTER(WHERE user_id IS ?1) FROM chat_connections")
            .bind(user)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    if total >= 512 || own >= 32 {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    sqlx::query("INSERT INTO chat_connections(id,room_id,user_id,expires_at)VALUES(?1,?2,?3,unixepoch('subsec')+20)").bind(id).bind(room).bind(user).execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
}
pub(super) async fn snapshot(pool: &SqlitePool, room: &str) -> Result<Vec<Uuid>, sqlx::Error> {
    sqlx::query_scalar("SELECT id FROM chat_connections WHERE room_id=?1 AND expires_at>unixepoch('subsec') ORDER BY id LIMIT 512").bind(room).fetch_all(pool).await
}
pub(super) async fn presence(
    State(state): State<StateData>,
    headers: HeaderMap,
    Path(room): Path<String>,
) -> Result<Json<Vec<Uuid>>, StatusCode> {
    let actor = auth::principal(&state, &headers, false).await?;
    auth::room_allowed(&state, actor, &room).await?;
    snapshot(&state.pool, &room)
        .await
        .map(Json)
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
}
