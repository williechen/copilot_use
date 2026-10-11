//! Chat 可靠事件 dispatcher 獨立 Worker（HTTP 僅使用 HTTPS 或本機 loopback）。
use sqlx::postgres::PgPoolOptions;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_url = std::env::var("CHAT_DATABASE_URL")?;
    let endpoint = std::env::var("ANALYTICS_ENDPOINT")?;
    let token = std::env::var("ANALYTICS_API_TOKEN")?;
    if !(endpoint.starts_with("https://") || endpoint.starts_with("http://127.0.0.1:") || endpoint.starts_with("http://localhost:")) {
        return Err("ANALYTICS_ENDPOINT must be https:// or local loopback".into());
    }
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    day9_chat::run_dispatcher(pool, &endpoint, &token).await
}
