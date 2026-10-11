//! RAG Adapter：明確加入／刪除資料、owner-scoped pgvector 檢索與可追溯引用。
use super::{
    StateData,
    auth::{self, Principal},
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Document {
    title: String,
    content: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Search {
    query: String,
}
#[derive(Serialize, FromRow)]
pub(super) struct Hit {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub distance: f64,
}
fn vector(value: &[f32]) -> String {
    format!(
        "[{}]",
        value
            .iter()
            .map(f32::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}
pub(super) async fn add(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<Document>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let actor = auth::principal(&state, &headers, true).await?;
    if input.title.trim().is_empty()
        || input.title.len() > 200
        || input.content.trim().is_empty()
        || input.content.len() > 32768
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let _permit = state
        .ai_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;
    let embedding = openai_adapter::embedding(&input.content)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO knowledge_documents(id,owner_id,title,content,embedding) VALUES ($1,$2,$3,$4,$5::vector)").bind(id).bind(actor.user_id()).bind(input.title).bind(input.content).bind(vector(&embedding)).execute(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok((StatusCode::CREATED, Json(serde_json::json!({"id":id}))))
}
pub(super) async fn list(
    State(state): State<StateData>,
    headers: HeaderMap,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let actor = auth::principal(&state, &headers, false).await?;
    let rows:Vec<(Uuid,String)>=sqlx::query_as("SELECT id,title FROM knowledge_documents WHERE owner_id IS NOT DISTINCT FROM $1 ORDER BY created_at,id LIMIT 100").bind(actor.user_id()).fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(
        rows.into_iter()
            .map(|(id, title)| serde_json::json!({"id":id,"title":title}))
            .collect(),
    ))
}
pub(super) async fn delete(
    State(state): State<StateData>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let actor = auth::principal(&state, &headers, true).await?;
    let result = sqlx::query(
        "DELETE FROM knowledge_documents WHERE id=$1 AND owner_id IS NOT DISTINCT FROM $2",
    )
    .bind(id)
    .bind(actor.user_id())
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    if result.rows_affected() == 1 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
pub(super) async fn retrieve(
    state: &StateData,
    actor: Principal,
    query: &str,
) -> Result<Vec<Hit>, StatusCode> {
    if query.trim().is_empty() || query.len() > 8192 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let value = openai_adapter::embedding(query)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    sqlx::query_as("SELECT id,title,content,(embedding <=> $1::vector)::float8 AS distance FROM knowledge_documents WHERE owner_id IS NOT DISTINCT FROM $2 ORDER BY embedding <=> $1::vector LIMIT 4")
  .bind(vector(&value)).bind(actor.user_id()).fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)
}
pub(super) async fn search(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<Search>,
) -> Result<Json<Vec<Hit>>, StatusCode> {
    let actor = auth::principal(&state, &headers, true).await?;
    let _permit = state
        .ai_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;
    Ok(Json(retrieve(&state, actor, &input.query).await?))
}
