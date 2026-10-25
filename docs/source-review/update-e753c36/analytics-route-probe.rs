use analytics_server::{EventInput, IngestState, IngestTokens};
use axum::{body::{Body, to_bytes}, http::Request};
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::PgPool::connect("postgresql://willie@localhost/codex_review_e753c36?host=/var/run/postgresql").await?;
    sqlx::migrate!("../rust_ai_ring/analytics_apps/migrations").run(&pool).await?;
    let event = EventInput { schema_version: 1, event_id: uuid::Uuid::new_v4(), source_app: "web_chat".into(), event_type: "codex.review.synthetic".into() };
    analytics_server::pg_store::persist_event(&pool, &event).await.map_err(|_| "synthetic fixture insert failed")?;
    let app = analytics_server::build_app(IngestState { pool, tokens: Arc::new(IngestTokens { chat: "review-fixture-chat-only".into(), pet: "review-fixture-pet-only".into() }) });
    for (case, path, bearer, expected) in [
        ("exact_no_cookie_no_token", "/api/v1/events/web_chat/summary/codex.review.synthetic", None, 401),
        ("exact_no_cookie_chat_token", "/api/v1/events/web_chat/summary/codex.review.synthetic", Some("review-fixture-chat-only"), 200),
        ("exact_no_cookie_pet_token", "/api/v1/events/web_chat/summary/codex.review.synthetic", Some("review-fixture-pet-only"), 401),
        ("full_summary_no_cookie_chat_token", "/api/v1/events/web_chat/summary", Some("review-fixture-chat-only"), 401),
    ] {
        let mut request = Request::get(path);
        if let Some(token) = bearer { request = request.header("authorization", format!("Bearer {token}")); }
        let response = app.clone().oneshot(request.body(Body::empty())?).await?;
        let status = response.status().as_u16();
        let bytes = to_bytes(response.into_body(), 8192).await?;
        println!("{case}: HTTP {status}; body={}", String::from_utf8_lossy(&bytes));
        assert_eq!(status, expected);
    }
    Ok(())
}
