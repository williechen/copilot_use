//! Authenticated MCP host. Tool arguments never choose another user's data scope.
use super::{StateData, auth, knowledge};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde_json::{Value, json};
pub(super) async fn handler(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(value): Json<Value>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let actor = auth::principal(&state, &headers, true).await?;
    if let Some(origin) = headers.get("origin").and_then(|v| v.to_str().ok()) {
        let parsed = url::Url::parse(origin).map_err(|_| StatusCode::FORBIDDEN)?;
        let host = headers
            .get("host")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();
        let expected = parsed
            .host_str()
            .map(|h| match parsed.port() {
                Some(p) => format!("{h}:{p}"),
                None => h.into(),
            })
            .unwrap_or_default();
        if expected != host || !matches!(parsed.scheme(), "http" | "https") {
            return Err(StatusCode::FORBIDDEN);
        }
    }
    let id = value["id"].clone();
    let request: mcp_bridge::Request = match serde_json::from_value(value) {
        Ok(r) => r,
        Err(_) => {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(mcp_bridge::error(id, -32600, "Invalid request")),
            ));
        }
    };
    if let Err((code, message)) = request.validate_http(&headers) {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(mcp_bridge::error(request.id, code, message)),
        ));
    }
    let response = match request.method.as_str() {
        "server/discover" => mcp_bridge::discover(),
        "tools/list" => {
            json!({"tools":[mcp_bridge::tool("knowledge_search","Search only the caller's explicitly added reference documents; no file or network commands.")],"cacheScope":"private","ttlMs":0})
        }
        "tools/call" => {
            if request.params["name"] != "knowledge_search" {
                return Ok((
                    StatusCode::BAD_REQUEST,
                    Json(mcp_bridge::error(request.id, -32602, "Tool not permitted")),
                ));
            }
            let arguments = &request.params["arguments"];
            if arguments.as_object().is_none_or(|p| p.len() != 1)
                || arguments["query"]
                    .as_str()
                    .is_none_or(|q| q.trim().is_empty() || q.len() > 4096)
            {
                return Ok((
                    StatusCode::BAD_REQUEST,
                    Json(mcp_bridge::error(request.id, -32602, "Invalid arguments")),
                ));
            }
            let _permit = state
                .tool_slots
                .clone()
                .try_acquire_owned()
                .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;
            let hits =
                knowledge::retrieve(&state, actor, arguments["query"].as_str().unwrap()).await?;
            let hits:Vec<Value>=hits.into_iter().map(|hit|json!({"id":hit.id,"title":hit.title,"content":hit.content.chars().take(4096).collect::<String>()})).collect();
            mcp_bridge::tool_result(json!({"hits":hits}))
        }
        _ => {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(mcp_bridge::error(
                    request.id,
                    -32601,
                    "Method not found; supported protocol 2026-07-28",
                )),
            ));
        }
    };
    Ok((
        StatusCode::OK,
        Json(mcp_bridge::result(request.id, response, "copilot-chat")),
    ))
}
