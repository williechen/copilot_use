//! Chat HTTP API 與 Outbox Dispatcher；完整 WebSocket/Leptos UI 尚待整合。
use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use day9_chat::{ChatWriter, run_dispatcher};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{error::Error, net::SocketAddr};

#[derive(Clone)]
struct StateData { pool: PgPool }
#[derive(Deserialize)]
struct RoomRequest { id: String, name: String }
#[derive(Deserialize)]
struct MessageRequest { room_id: String, content: String }
#[derive(Serialize)]
struct CreatedMessage { id: String }

async fn health() -> StatusCode { StatusCode::OK }

async fn create_room(State(s): State<StateData>, Json(input): Json<RoomRequest>)
    -> Result<StatusCode, StatusCode>
{
    if input.id.is_empty() || input.id.len() > 40 || input.name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    sqlx::query("INSERT INTO chat_rooms(id,name) VALUES($1,$2) ON CONFLICT(id) DO NOTHING")
        .bind(&input.id).bind(&input.name).execute(&s.pool).await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(StatusCode::CREATED)
}

async fn send_message(State(s): State<StateData>, Json(input): Json<MessageRequest>)
    -> Result<(StatusCode, Json<CreatedMessage>), StatusCode>
{
    if input.room_id.is_empty() || input.room_id.len() > 40 ||
       input.content.trim().is_empty() || input.content.len() > 65536 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let id = ChatWriter::new(s.pool).persist(&input.room_id, &input.content).await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok((StatusCode::CREATED, Json(CreatedMessage { id: id.to_string() })))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = std::env::var("CHAT_DATABASE_URL")?;
    let endpoint = std::env::var("ANALYTICS_ENDPOINT")?;
    let token = std::env::var("ANALYTICS_API_TOKEN")?;
    if token.len() < 16 || !(endpoint.starts_with("https://") ||
       endpoint.starts_with("http://127.0.0.1:") || endpoint.starts_with("http://localhost:")) {
        return Err("Analytics endpoint 必須 HTTPS/loopback，且 Token 至少 16 字元".into());
    }
    let pool = PgPoolOptions::new().max_connections(10).connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let worker_pool = pool.clone();
    let worker = tokio::spawn(async move {
        if let Err(error) = run_dispatcher(worker_pool, &endpoint, &token).await {
            eprintln!("outbox dispatcher error: {error}");
        }
    });
    let app = Router::new().route("/healthz", get(health))
        .route("/api/v1/rooms", post(create_room))
        .route("/api/v1/messages", post(send_message))
        .layer(axum::extract::DefaultBodyLimit::max(65536))
        .with_state(StateData { pool });
    let addr: SocketAddr = std::env::var("CHAT_LISTEN")
        .unwrap_or_else(|_| "127.0.0.1:3001".into()).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Chat listening on {addr}");
    let result = axum::serve(listener, app).with_graceful_shutdown(async {
        let _ = tokio::signal::ctrl_c().await;
    }).await;
    worker.abort();
    result?;
    Ok(())
}