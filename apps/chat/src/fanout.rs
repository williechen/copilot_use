//! Cross-process notifications carry only message IDs; canonical content is fetched after commit.
use super::{StateData, auth, realtime::RoomEvent};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde_json::{Value, json};
use sqlx::{PgPool, postgres::PgListener};
use tokio::sync::broadcast;
use uuid::Uuid;
pub(super) async fn start(
    url: &str,
    pool: PgPool,
    events: broadcast::Sender<RoomEvent>,
) -> Result<tokio::task::JoinHandle<()>, sqlx::Error> {
    let mut listener = PgListener::connect(url).await?;
    listener.listen("copilot_chat_realtime").await?;
    Ok(tokio::spawn(async move {
        loop {
            match listener.recv().await {
                Ok(notification) => {
                    let Ok(value) = serde_json::from_str::<Value>(notification.payload()) else {
                        continue;
                    };
                    let Some(room) = value["room_id"]
                        .as_str()
                        .filter(|s| !s.is_empty() && s.len() <= 40)
                    else {
                        continue;
                    };
                    if value["type"] == "message" {
                        let Some(id) = value["id"].as_str().and_then(|s| Uuid::parse_str(s).ok())
                        else {
                            continue;
                        };
                        let row: Result<Option<String>, _> = sqlx::query_scalar(
                            "SELECT content FROM chat_messages WHERE id=$1 AND room_id=$2",
                        )
                        .bind(id)
                        .bind(room)
                        .fetch_optional(&pool)
                        .await;
                        if let Ok(Some(content)) = row {
                            let _ = events.send(RoomEvent {
                                room_id: room.into(),
                                payload: json!({"type":"message","id":id,"content":content}),
                            });
                        } else {
                            let _ = events.send(RoomEvent {
                                room_id: room.into(),
                                payload: json!({"type":"resync_required"}),
                            });
                        }
                    } else if matches!(value["type"].as_str(), Some("presence" | "typing"))
                        && value["connection_id"]
                            .as_str()
                            .and_then(|s| Uuid::parse_str(s).ok())
                            .is_some()
                    {
                        let mut payload = value.clone();
                        payload.as_object_mut().unwrap().remove("room_id");
                        let _ = events.send(RoomEvent {
                            room_id: room.into(),
                            payload,
                        });
                    }
                }
                Err(_) => {
                    let _ = events.send(RoomEvent {
                        room_id: "*".into(),
                        payload: json!({"type":"resync_required"}),
                    });
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
            }
        }
    }))
}
pub(super) async fn publish(
    pool: &PgPool,
    room: &str,
    mut payload: Value,
) -> Result<(), sqlx::Error> {
    payload["room_id"] = json!(room);
    sqlx::query("SELECT pg_notify('copilot_chat_realtime',$1)")
        .bind(payload.to_string())
        .execute(pool)
        .await?;
    Ok(())
}
pub(super) async fn join(
    pool: &PgPool,
    room: &str,
    id: Uuid,
    user: Option<Uuid>,
) -> Result<(), StatusCode> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("SELECT pg_advisory_xact_lock(71420391)")
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("DELETE FROM chat_connections WHERE expires_at<=now()")
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let (total,own):(i64,i64)=sqlx::query_as("SELECT count(*)::bigint,count(*)FILTER(WHERE user_id IS NOT DISTINCT FROM $1)::bigint FROM chat_connections").bind(user).fetch_one(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    if total >= 512 || own >= 32 {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    sqlx::query("INSERT INTO chat_connections(id,room_id,user_id,expires_at)VALUES($1,$2,$3,now()+interval '20 seconds')").bind(id).bind(room).bind(user).execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
}
pub(super) async fn snapshot(pool: &PgPool, room: &str) -> Result<Vec<Uuid>, sqlx::Error> {
    sqlx::query_scalar("SELECT id FROM chat_connections WHERE room_id=$1 AND expires_at>now() ORDER BY id LIMIT 512").bind(room).fetch_all(pool).await
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
