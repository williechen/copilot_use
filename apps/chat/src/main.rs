//! Chat Web、HTTP、PostgreSQL Transactional Outbox；獨立測試庫與認證。
mod openai;
mod realtime;
mod web;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    routing::get,
};
use day9_chat::{ChatWriter, run_dispatcher};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use std::{error::Error, net::SocketAddr, sync::Arc};

#[derive(Clone)]
struct StateData {
    pool: PgPool,
    api_token: Arc<str>,
    events: tokio::sync::broadcast::Sender<realtime::RoomEvent>,
    ai_slots: Arc<tokio::sync::Semaphore>,
}
#[derive(Deserialize)]
struct RoomRequest {
    id: String,
    name: String,
}
#[derive(Deserialize)]
struct MessageRequest {
    room_id: String,
    content: String,
}
#[derive(Deserialize)]
struct MessagesQuery {
    room_id: String,
}
#[derive(Serialize)]
struct CreatedMessage {
    id: String,
}
#[derive(Serialize, FromRow)]
struct RoomItem {
    id: String,
    name: String,
}
#[derive(Serialize, FromRow)]
struct MessageItem {
    id: String,
    content: String,
}

fn authorized(headers: &HeaderMap, token: &str) -> bool {
    headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .is_some_and(|v| !token.is_empty() && v == token)
}

async fn health(State(s): State<StateData>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&s.pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

async fn list_rooms(
    State(s): State<StateData>,
    headers: HeaderMap,
) -> Result<Json<Vec<RoomItem>>, StatusCode> {
    if !authorized(&headers, &s.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let rooms = sqlx::query_as::<_, RoomItem>(
        "SELECT id, name FROM chat_rooms ORDER BY name, id LIMIT 100",
    )
    .fetch_all(&s.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(rooms))
}

async fn create_room(
    State(s): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<RoomRequest>,
) -> Result<StatusCode, StatusCode> {
    if !authorized(&headers, &s.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if input.id.is_empty()
        || input.id.len() > 40
        || input.name.trim().is_empty()
        || input.name.len() > 200
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    sqlx::query("INSERT INTO chat_rooms(id,name) VALUES($1,$2) ON CONFLICT(id) DO NOTHING")
        .bind(&input.id)
        .bind(&input.name)
        .execute(&s.pool)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(StatusCode::CREATED)
}

async fn list_messages(
    State(s): State<StateData>,
    headers: HeaderMap,
    Query(input): Query<MessagesQuery>,
) -> Result<Json<Vec<MessageItem>>, StatusCode> {
    if !authorized(&headers, &s.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if input.room_id.is_empty() || input.room_id.len() > 40 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let messages = sqlx::query_as::<_, MessageItem>(
        "SELECT id::text AS id, content FROM (
          SELECT id, content, created_at FROM chat_messages
          WHERE room_id=$1 ORDER BY created_at DESC, id DESC LIMIT 100
         ) AS recent ORDER BY created_at ASC, id ASC",
    )
    .bind(&input.room_id)
    .fetch_all(&s.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(messages))
}

async fn send_message(
    State(s): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<MessageRequest>,
) -> Result<(StatusCode, Json<CreatedMessage>), StatusCode> {
    if !authorized(&headers, &s.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if input.room_id.is_empty()
        || input.room_id.len() > 40
        || input.content.trim().is_empty()
        || input.content.len() > 65536
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let id = ChatWriter::new(s.pool.clone())
        .persist(&input.room_id, &input.content)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let _ = s.events.send(realtime::RoomEvent {
        room_id: input.room_id,
        payload: serde_json::json!({"type":"message", "id": id, "content": input.content}),
    });
    Ok((
        StatusCode::CREATED,
        Json(CreatedMessage { id: id.to_string() }),
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = std::env::var("CHAT_DATABASE_URL")?;
    let endpoint = std::env::var("ANALYTICS_ENDPOINT")?;
    let token = std::env::var("ANALYTICS_CHAT_TOKEN")?;
    let chat_token = std::env::var("CHAT_API_TOKEN")?;
    let isolated_docker_dev =
        std::env::var("ALLOW_INTERNAL_HTTP_DEV").is_ok_and(|value| value == "true");
    if token.len() < 16
        || chat_token.len() < 16
        || !(endpoint.starts_with("https://")
            || endpoint.starts_with("http://127.0.0.1:")
            || endpoint.starts_with("http://localhost:")
            || (isolated_docker_dev && endpoint == "http://analytics:3100/api/v1/events"))
    {
        return Err("須設定安全 Token 和 HTTPS/localhost Analytics endpoint".into());
    }
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(10)
        .connect(&url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let worker_pool = pool.clone();
    let worker = tokio::spawn(async move {
        if let Err(error) = run_dispatcher(worker_pool, &endpoint, &token).await {
            eprintln!("outbox dispatcher error: {error}");
        }
    });
    let (events, _) = tokio::sync::broadcast::channel(256);
    let app = Router::new()
        .route("/", get(web::home))
        .route("/healthz", get(health))
        .route("/api/v1/ws/{room_id}", get(realtime::upgrade))
        .route("/api/v1/ai/reply", axum::routing::post(openai::reply))
        .route("/api/v1/rooms", get(list_rooms).post(create_room))
        .route("/api/v1/messages", get(list_messages).post(send_message))
        .layer(axum::extract::DefaultBodyLimit::max(65536))
        .with_state(StateData {
            pool,
            api_token: Arc::from(chat_token),
            events,
            ai_slots: Arc::new(tokio::sync::Semaphore::new(2)),
        });
    let addr: SocketAddr = std::env::var("CHAT_LISTEN")
        .unwrap_or_else(|_| "127.0.0.1:3001".into())
        .parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Chat listening on {addr}");
    let result = axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await;
    worker.abort();
    result?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_bearer_token_is_denied() {
        assert!(!authorized(&HeaderMap::new(), "long-secret-token"));
    }
}
