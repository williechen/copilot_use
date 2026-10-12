//! HTTP Adapter: callers choose only their own consent in an accessible room.
use super::{StateData, auth};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::Value;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Consent {
    enabled: bool,
}
pub(super) async fn get(
    State(state): State<StateData>,
    headers: HeaderMap,
    Path(room): Path<String>,
) -> Result<Response, StatusCode> {
    let actor = auth::principal(&state, &headers, false).await?;
    let user = actor.user_id().ok_or(StatusCode::FORBIDDEN)?;
    auth::room_allowed(&state, actor, &room).await?;
    let value = day9_chat::topics::consent(&state.pool, user, &room)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let mut response = Json(value).into_response();
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store"),
    );
    Ok(response)
}
pub(super) async fn set(
    State(state): State<StateData>,
    headers: HeaderMap,
    Path(room): Path<String>,
    Json(input): Json<Consent>,
) -> Result<Json<Value>, StatusCode> {
    let actor = auth::principal(&state, &headers, true).await?;
    let user = actor.user_id().ok_or(StatusCode::FORBIDDEN)?;
    auth::room_allowed(&state, actor, &room).await?;
    day9_chat::topics::set_consent(&state.pool, user, &room, input.enabled)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(
        day9_chat::topics::consent(&state.pool, user, &room)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?,
    ))
}
