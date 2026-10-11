//! Analytics Web/API 同一服務啟動入口，使用獨立 ANALYTICS_DATABASE_URL。
use day9_analytics::{AnalyticsState, build_app};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("ANALYTICS_DATABASE_URL")?;
    let token = std::env::var("ANALYTICS_API_TOKEN")?;
    if token.len() < 16 { return Err("ANALYTICS_API_TOKEN must contain >=16 chars".into()); }
    let pool = PgPoolOptions::new().max_connections(5).connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let app = build_app(AnalyticsState { pool, api_token: Arc::from(token) });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3100").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
