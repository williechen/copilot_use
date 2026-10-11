//! 房間隔離的即時通知；瀏覽器在第一個 frame 驗證，不把 Token 放 URL。
use super::{StateData, auth};
use axum::{
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::{HeaderMap, StatusCode},
    response::Response,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone)]
pub(super) struct RoomEvent {
    pub room_id: String,
    pub payload: Value,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Authentication {
    token: String,
}

pub(super) async fn upgrade(
    State(state): State<StateData>,
    Path(room_id): Path<String>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Result<Response, StatusCode> {
    if room_id.is_empty() || room_id.len() > 40 {
        return Err(StatusCode::BAD_REQUEST);
    }
    // Browser WS 只能從同一 origin 連入；非 Browser client 仍須第一 frame 驗證。
    if let Some(origin) = headers.get("origin") {
        let host = headers
            .get("host")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::FORBIDDEN)?;
        let origin = origin.to_str().map_err(|_| StatusCode::FORBIDDEN)?;
        if origin != format!("http://{host}") && origin != format!("https://{host}") {
            return Err(StatusCode::FORBIDDEN);
        }
    }
    let cookie_actor = if headers.contains_key("cookie") {
        let actor = auth::principal(&state, &headers, false).await?;
        auth::room_allowed(&state, actor, &room_id).await?;
        Some(actor)
    } else {
        None
    };
    Ok(ws
        .max_message_size(4096)
        .max_frame_size(4096)
        .on_upgrade(move |socket| connected(socket, state, room_id, cookie_actor)))
}
async fn send(socket: &mut WebSocket, value: Value) -> bool {
    tokio::time::timeout(
        Duration::from_secs(5),
        socket.send(Message::Text(value.to_string().into())),
    )
    .await
    .is_ok_and(|r| r.is_ok())
}
async fn connected(
    mut socket: WebSocket,
    state: StateData,
    room_id: String,
    cookie_actor: Option<auth::Principal>,
) {
    let auth = tokio::time::timeout(Duration::from_secs(5), socket.recv()).await;
    let authenticated = cookie_actor.is_some()
        || match auth {
            Ok(Some(Ok(Message::Text(value)))) => serde_json::from_str::<Authentication>(&value)
                .is_ok_and(|a| !state.api_token.is_empty() && a.token == state.api_token.as_ref()),
            _ => false,
        };
    if !authenticated {
        send(&mut socket, json!({"type":"error","code":"unauthorized"})).await;
        let _ = socket.send(Message::Close(None)).await;
        return;
    }
    let exists: Result<bool, _> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM chat_rooms WHERE id=$1)")
            .bind(&room_id)
            .fetch_one(&state.pool)
            .await;
    if !matches!(exists, Ok(true)) {
        send(
            &mut socket,
            json!({"type":"error","code":"room_unavailable"}),
        )
        .await;
        let _ = socket.send(Message::Close(None)).await;
        return;
    }
    let mut subscription = state.events.subscribe();
    let connection = Uuid::new_v4();
    if !send(
        &mut socket,
        json!({"type":"ready","connection_id":connection}),
    )
    .await
    {
        return;
    }
    let _ = state.events.send(RoomEvent {
        room_id: room_id.clone(),
        payload: json!({"type":"presence","connection_id":connection,"online":true}),
    });
    let mut last_typing = tokio::time::Instant::now() - Duration::from_secs(1);
    loop {
        tokio::select! {
            incoming = socket.recv() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        if text.as_str() == r#"{"type":"typing"}"# && last_typing.elapsed() >= Duration::from_millis(500) {
                            last_typing = tokio::time::Instant::now();
                            let _ = state.events.send(RoomEvent {room_id:room_id.clone(), payload:json!({"type":"typing","connection_id":connection})});
                        } else if !send(&mut socket, json!({"type":"error","code":"unsupported_or_rate_limited"})).await {break;}
                    }
                    Some(Ok(Message::Ping(value))) => {if socket.send(Message::Pong(value)).await.is_err() {break;}}
                    Some(Ok(Message::Pong(_))) => {}
                    _ => break,
                }
            }
            event = subscription.recv() => {
                match event {
                    Ok(event) if event.room_id == room_id => {
                        if !send(&mut socket, event.payload).await {break;}
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        if !send(&mut socket,json!({"type":"resync_required"})).await {break;}
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                    _ => {}
                }
            }
        }
    }
    let _ = state.events.send(RoomEvent {
        room_id,
        payload: json!({"type":"presence","connection_id":connection,"online":false}),
    });
}
