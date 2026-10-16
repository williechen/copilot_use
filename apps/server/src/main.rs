#![recursion_limit = "256"]
//! Chat Web、HTTP、SQLite Transactional Outbox；獨立測試庫與認證。
mod agent;
mod audit;
mod auth;
mod fanout;
mod knowledge;
mod management;
mod mcp;
mod openai;
mod personas;
mod realtime;
mod suggestions;
mod topic_http;
mod web;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    routing::get,
};
use day9_chat::{ChatWriter, run_dispatcher};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::{error::Error, net::SocketAddr, sync::Arc};

#[derive(Clone)]
struct StateData {
    pool: SqlitePool,
    api_token: Arc<str>,
    audit_token: Arc<str>,
    events: tokio::sync::broadcast::Sender<realtime::RoomEvent>,
    ai_slots: Arc<tokio::sync::Semaphore>,
    auth_slots: Arc<tokio::sync::Semaphore>,
    ws_slots: Arc<tokio::sync::Semaphore>,
    tool_slots: Arc<tokio::sync::Semaphore>,
    mcp_endpoint: Arc<str>,
}
#[derive(Deserialize)]
struct RoomRequest {
    id: String,
    name: String,
    #[serde(default)]
    is_private: bool,
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
    before: Option<uuid::Uuid>,
}
#[derive(Serialize)]
struct CreatedMessage {
    id: String,
}
#[derive(Serialize, FromRow)]
struct RoomItem {
    id: String,
    name: String,
    can_manage: bool,
    is_private: bool,
}
#[derive(Serialize, FromRow)]
struct MessageItem {
    id: uuid::Uuid,
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
        "SELECT r.id,r.name,COALESCE(?1 IS NULL OR r.owner_id=?1,false) AS can_manage,r.is_private FROM chat_rooms r WHERE ?1 IS NULL OR NOT r.is_private OR EXISTS(SELECT 1 FROM chat_room_members m WHERE m.room_id=r.id AND m.user_id=?1) ORDER BY r.name,r.id LIMIT 100",
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
        || !input
            .id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
        || input.name.trim().is_empty()
        || input.name.len() > 200
        || input.name.contains('\0')
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut tx = s
        .pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("INSERT INTO chat_rooms(id,name,owner_id,is_private) VALUES(?1,?2,?3,?4)")
        .bind(&input.id)
        .bind(&input.name)
        .bind(actor.user_id())
        .bind(input.is_private)
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
        sqlx::query("INSERT INTO chat_room_members(room_id,user_id) VALUES (?1,?2)")
            .bind(&input.id)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    }
    let username: Option<String> =
        sqlx::query_scalar("SELECT username FROM chat_users WHERE id=?1")
            .bind(actor.user_id())
            .fetch_optional(&mut *tx)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let event_id = uuid::Uuid::new_v4();
    sqlx::query("INSERT INTO chat_outbox(event_id,schema_version,source_app,event_type,traceparent)VALUES(?1,1,'web_chat','room.created',?2)").bind(event_id).bind(copilot_telemetry::outgoing_headers().get("traceparent")).execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    audit::record(
        &mut tx,
        audit::Entry {
            id: event_id,
            kind: "room.created",
            user_id: actor.user_id(),
            username: username.as_deref().unwrap_or(""),
            room_id: Some(&input.id),
            room_name: Some(&input.name),
            target_username: None,
        },
    )
    .await?;
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
    if let Some(cursor) = input.before {
        let valid: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM chat_messages WHERE id=?1 AND room_id=?2)",
        )
        .bind(cursor)
        .bind(&input.room_id)
        .fetch_one(&s.pool)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        if !valid {
            return Err(StatusCode::NOT_FOUND);
        }
    }
    let messages = sqlx::query_as::<_, MessageItem>(
        "SELECT id AS id, content FROM (
          SELECT id, content, created_at FROM chat_messages
          WHERE room_id=?1 AND (?2 IS NULL OR (created_at,id)<(SELECT created_at,id FROM chat_messages WHERE id=?2 AND room_id=?1))
          ORDER BY created_at DESC, id DESC LIMIT 100
         ) AS recent ORDER BY created_at ASC, id ASC",
    )
    .bind(&input.room_id).bind(input.before)
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
    if input.id.is_some_and(|id| id.is_nil())
        || input.room_id.is_empty()
        || input.room_id.len() > 40
        || input.content.trim().is_empty()
        || input.content.len() > 65536
        || input.content.contains('\0')
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let user = actor.user_id().map(|id| id.to_string());
    let committed = day9_chat::application::persist(
        &ChatWriter::new(s.pool.clone()),
        day9_chat::application::MessageInput {
            id: input.id.unwrap_or_else(uuid::Uuid::new_v4),
            room: &input.room_id,
            content: &input.content,
            author_kind: if user.is_some() {
                "user"
            } else {
                "anonymous_user"
            },
            author_id: user.as_deref(),
        },
    )
    .await
    .map_err(|error| match error {
        day9_chat::application::MessageError::InvalidInput => StatusCode::BAD_REQUEST,
        day9_chat::application::MessageError::Conflict => StatusCode::CONFLICT,
        day9_chat::application::MessageError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
    })?;
    let id = committed.id;
    let inserted = committed.inserted;
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
    let url =
        std::env::var("CHAT_DATABASE_URL").unwrap_or_else(|_| "sqlite://data/chat.sqlite3".into());
    if std::env::args().nth(1).as_deref() == Some("--initialize") {
        let pool = day9_chat::database::open(&url).await?;
        pool.close().await;
        return Ok(());
    }
    if std::env::args().nth(1).as_deref() == Some("--backup") {
        let destination = std::env::args()
            .nth(2)
            .ok_or("usage: day9-chat --backup NEW_PATH.sqlite3")?;
        let pool = day9_chat::database::open(&url).await?;
        day9_chat::database::backup(&pool, std::path::Path::new(&destination)).await?;
        pool.close().await;
        return Ok(());
    }
    let endpoint = std::env::var("ANALYTICS_ENDPOINT")?;
    let token = std::env::var("ANALYTICS_CHAT_TOKEN")?;
    let chat_token = std::env::var("CHAT_API_TOKEN")?;
    let audit_token = std::env::var("CHAT_AUDIT_TOKEN").unwrap_or_default();
    if !audit_token.is_empty()
        && (audit_token.len() < 16 || audit_token == chat_token || audit_token == token)
    {
        return Err("稽核 Token 必須獨立且至少16字元".into());
    }
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
    let pool = day9_chat::database::open(&url).await?;
    let worker_pool = pool.clone();
    let worker = tokio::spawn(async move {
        if let Err(error) = run_dispatcher(worker_pool, &endpoint, &token).await {
            eprintln!("outbox dispatcher error: {error}");
        }
    });
    let (events, _) = tokio::sync::broadcast::channel(256);
    let fanout_task = fanout::start(&url, pool.clone(), events.clone()).await?;
    let addr: SocketAddr = std::env::var("CHAT_LISTEN")
        .unwrap_or_else(|_| "127.0.0.1:3001".into())
        .parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let bound = listener.local_addr()?;
    let mcp_endpoint: Arc<str> = format!(
        "http://{}:{}/api/v1/mcp",
        if bound.is_ipv6() {
            "[::1]"
        } else {
            "127.0.0.1"
        },
        bound.port()
    )
    .into();
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
        .route(
            "/api/v1/rooms/{room}/topic-consent",
            get(topic_http::get).post(topic_http::set),
        )
        .route("/api/v1/mcp", axum::routing::post(mcp::handler))
        .route("/api/v1/auth/register", axum::routing::post(auth::register))
        .route("/api/v1/auth/login", axum::routing::post(auth::login))
        .route("/api/v1/auth/logout", axum::routing::post(auth::logout))
        .route("/api/v1/rooms/invite", axum::routing::post(auth::invite))
        .route("/api/v1/ws/{room_id}", get(realtime::upgrade))
        .route("/api/v1/rooms/{room}/presence", get(fanout::presence))
        .route(
            "/api/v1/rooms/{room}/personas",
            get(personas::list).post(personas::save),
        )
        .route(
            "/api/v1/rooms/{room}/persona-runs",
            axum::routing::post(personas::run),
        )
        .route("/api/v1/rooms/{room}/persona-runs/{id}", get(personas::get))
        .route(
            "/api/v1/rooms/{room}/persona-runs/{id}/cancel",
            axum::routing::post(personas::cancel),
        )
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
        .route(
            "/api/v1/rooms/{room}/suggestions/{id}/apply",
            axum::routing::post(suggestions::apply),
        )
        .route(
            "/api/v1/rooms/{room}/suggestions/{id}/rollback",
            axum::routing::post(suggestions::rollback),
        )
        .route("/api/v1/ai/agent", axum::routing::post(agent::reply))
        .route("/api/v1/ai/reply", axum::routing::post(openai::reply))
        .route(
            "/api/v1/rooms/{id}/join",
            axum::routing::post(management::join),
        )
        .route("/api/v1/me/rooms", get(management::rooms))
        .route("/api/v1/me/rooms/{id}/members", get(management::members))
        .route("/api/v1/me/activity", axum::routing::post(audit::mine))
        .route("/api/v1/audit/activity", axum::routing::post(audit::query))
        .route("/api/v1/rooms", get(list_rooms).post(create_room))
        .route("/api/v1/messages", get(list_messages).post(send_message))
        .layer(axum::extract::DefaultBodyLimit::max(65536))
        .layer(axum::middleware::from_fn(copilot_telemetry::http))
        .with_state(StateData {
            pool,
            api_token: Arc::from(chat_token),
            audit_token: Arc::from(audit_token),
            events,
            ai_slots: Arc::new(tokio::sync::Semaphore::new(2)),
            auth_slots: Arc::new(tokio::sync::Semaphore::new(4)),
            ws_slots: Arc::new(tokio::sync::Semaphore::new(128)),
            tool_slots: Arc::new(tokio::sync::Semaphore::new(2)),
            mcp_endpoint,
        });
    println!("Chat listening on {addr}");
    let result = axum::serve(listener, app)
        .with_graceful_shutdown(async {
            copilot_telemetry::shutdown_signal().await;
        })
        .await;
    worker.abort();
    fanout_task.abort();
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
