//! Analytics Web/API 同一服務啟動入口，使用獨立 ANALYTICS_DATABASE_URL。
use day9_analytics::{AnalyticsState, build_app};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("ANALYTICS_DATABASE_URL")?;
    let token = std::env::var("ANALYTICS_API_TOKEN")?;
    let chat_token = std::env::var("ANALYTICS_CHAT_TOKEN")?;
    let pet_token = std::env::var("ANALYTICS_PET_TOKEN")?;
    if [token.as_str(), chat_token.as_str(), pet_token.as_str()]
        .iter()
        .any(|t| t.len() < 16)
        || token == chat_token
        || token == pet_token
        || chat_token == pet_token
    {
        return Err("ANALYTICS_API_TOKEN must contain >=16 chars".into());
    }
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(5)
        .connect(&url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let app = build_app(AnalyticsState {
        pool,
        api_token: Arc::from(token),
        chat_token: Arc::from(chat_token),
        pet_token: Arc::from(pet_token),
    });
    let address = std::env::var("ANALYTICS_LISTEN").unwrap_or_else(|_| "127.0.0.1:3100".into());
    let listener = tokio::net::TcpListener::bind(&address).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
