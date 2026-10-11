//! Chat Web、HTTP、PostgreSQL Transactional Outbox；獨立測試庫與認證。
mod auth;
mod knowledge;
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
    auth_slots: Arc<tokio::sync::Semaphore>,
}
#[derive(Deserialize)]
struct RoomRequest {
    id: String,
    name: String,
}
#[derive(Deserialize)]
struct MessageRequest {
    #[serde(default)]
    id: Option<uuid::Uuid>,
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
    let actor = auth::principal(&s, &headers, false).await?;
    let rooms = sqlx::query_as::<_, RoomItem>(
        "SELECT id,name FROM chat_rooms WHERE $1::uuid IS NULL OR id IN (SELECT room_id FROM chat_room_members WHERE user_id=$1) ORDER BY name,id LIMIT 100",
    )
    .bind(actor.user_id())
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
    let actor = auth::principal(&s, &headers, true).await?;
    if input.id.is_empty()
        || input.id.len() > 40
        || input.name.trim().is_empty()
        || input.name.len() > 200
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut tx = s
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("INSERT INTO chat_rooms(id,name,owner_id) VALUES($1,$2,$3)")
        .bind(&input.id)
        .bind(&input.name)
        .bind(actor.user_id())
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            if e.as_database_error()
                .is_some_and(|d| d.is_unique_violation())
            {
                StatusCode::CONFLICT
            } else {
                StatusCode::SERVICE_UNAVAILABLE
            }
        })?;
    if let Some(id) = actor.user_id() {
        sqlx::query("INSERT INTO chat_room_members(room_id,user_id) VALUES ($1,$2)")
            .bind(&input.id)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    }
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(StatusCode::CREATED)
}

async fn list_messages(
    State(s): State<StateData>,
    headers: HeaderMap,
    Query(input): Query<MessagesQuery>,
) -> Result<Json<Vec<MessageItem>>, StatusCode> {
    let actor = auth::principal(&s, &headers, false).await?;
    auth::room_allowed(&s, actor, &input.room_id).await?;
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
    let actor = auth::principal(&s, &headers, true).await?;
    auth::room_allowed(&s, actor, &input.room_id).await?;
    if input.room_id.is_empty()
        || input.room_id.len() > 40
        || input.content.trim().is_empty()
        || input.content.len() > 65536
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let user = actor.user_id().map(|id| id.to_string());
    let (id, inserted) = ChatWriter::new(s.pool.clone())
        .persist_with_id(
            input.id.unwrap_or_else(uuid::Uuid::new_v4),
            &input.room_id,
            &input.content,
            if user.is_some() {
                "user"
            } else {
                "anonymous_user"
            },
            user.as_deref(),
        )
        .await
        .map_err(|e| {
            if matches!(e, sqlx::Error::Protocol(_)) {
                StatusCode::CONFLICT
            } else {
                StatusCode::SERVICE_UNAVAILABLE
            }
        })?;
    if inserted {
        let _ = s.events.send(realtime::RoomEvent {
            room_id: input.room_id,
            payload: serde_json::json!({"type":"message", "id": id, "content": input.content}),
        });
    }
    Ok((
        if inserted {
            StatusCode::CREATED
        } else {
            StatusCode::OK
        },
        Json(CreatedMessage { id: id.to_string() }),
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = any_spawner::Executor::init_tokio();
    let telemetry = tokio::task::spawn_blocking(|| copilot_telemetry::init("chat")).await??;
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
        .route("/", get(web::leptos_home))
        .nest_service(
            "/pkg",
            tower_http::services::ServeDir::new(
                std::env::var("CHAT_SITE_DIR").unwrap_or_else(|_| "target/site/pkg".into()),
            ),
        )
        .fallback(web::leptos_home)
        .route("/healthz", get(health))
        .route("/api/v1/auth/register", axum::routing::post(auth::register))
        .route("/api/v1/auth/login", axum::routing::post(auth::login))
        .route("/api/v1/auth/logout", axum::routing::post(auth::logout))
        .route("/api/v1/rooms/invite", axum::routing::post(auth::invite))
        .route("/api/v1/ws/{room_id}", get(realtime::upgrade))
        .route(
            "/api/v1/knowledge",
            get(knowledge::list).post(knowledge::add),
        )
        .route(
            "/api/v1/knowledge/{id}",
            axum::routing::delete(knowledge::delete),
        )
        .route(
            "/api/v1/knowledge/search",
            axum::routing::post(knowledge::search),
        )
        .route("/api/v1/ai/reply", axum::routing::post(openai::reply))
        .route("/api/v1/rooms", get(list_rooms).post(create_room))
        .route("/api/v1/messages", get(list_messages).post(send_message))
        .layer(axum::extract::DefaultBodyLimit::max(65536))
        .layer(axum::middleware::from_fn(copilot_telemetry::http))
        .with_state(StateData {
            pool,
            api_token: Arc::from(chat_token),
            events,
            ai_slots: Arc::new(tokio::sync::Semaphore::new(2)),
            auth_slots: Arc::new(tokio::sync::Semaphore::new(4)),
        });
    let addr: SocketAddr = std::env::var("CHAT_LISTEN")
        .unwrap_or_else(|_| "127.0.0.1:3001".into())
        .parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Chat listening on {addr}");
    let result = axum::serve(listener, app)
        .with_graceful_shutdown(async {
            copilot_telemetry::shutdown_signal().await;
        })
        .await;
    worker.abort();
    result?;
    tokio::task::spawn_blocking(move || drop(telemetry)).await?;
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
