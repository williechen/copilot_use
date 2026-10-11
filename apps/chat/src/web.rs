//! Leptos SSR shell；與 WASM hydrate 共用同一個 Router。
pub async fn leptos_home(
    axum::extract::State(state): axum::extract::State<super::StateData>,
    request: axum::http::Request<axum::body::Body>,
) -> impl axum::response::IntoResponse {
    use axum::response::IntoResponse;
    if request.method() != axum::http::Method::GET {
        return axum::http::StatusCode::METHOD_NOT_ALLOWED.into_response();
    }
    if request.uri().path().starts_with("/api/") {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    }
    let signed_in = super::auth::principal(&state, request.headers(), false)
        .await
        .is_ok();
    let handler = leptos_axum::render_app_to_stream(move || chat_ui::shell(signed_in));
    handler(request).await.into_response()
}
