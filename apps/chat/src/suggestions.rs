//! User-confirmed scoped adoption and compare-and-swap rollback; remote acknowledgement is retryable.
use super::{StateData, auth};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Confirm {
    confirmed: bool,
}
fn remote() -> Result<(reqwest::Client, String, String), StatusCode> {
    let endpoint =
        std::env::var("ANALYTICS_ENDPOINT").map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let base = endpoint
        .strip_suffix("/api/v1/events")
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?
        .to_owned();
    let token =
        std::env::var("ANALYTICS_CHAT_TOKEN").map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let client = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok((client, base, token))
}
async fn owner(state: &StateData, headers: &HeaderMap, room: &str) -> Result<(), StatusCode> {
    let actor = auth::principal(state, headers, true).await?;
    let allowed: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM chat_rooms WHERE id=$1 AND ($2::uuid IS NULL OR owner_id=$2))",
    )
    .bind(room)
    .bind(actor.user_id())
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    if allowed {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
pub(super) async fn apply(
    State(state): State<StateData>,
    headers: HeaderMap,
    Path((room, id)): Path<(String, Uuid)>,
    Json(input): Json<Confirm>,
) -> Result<Json<Value>, StatusCode> {
    owner(&state, &headers, &room).await?;
    if !input.confirmed {
        return Err(StatusCode::BAD_REQUEST);
    }
    let (client, base, token) = remote()?;
    let response = client
        .get(format!("{base}/api/v1/suggestions/{id}"))
        .bearer_auth(&token)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    if !response.status().is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }
    let proposal: Value = response.json().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    if proposal["target_app"] != "web_chat"
        || proposal["proposal"]["action"] != "chat_context_messages"
        || !matches!(proposal["status"].as_str(), Some("approved" | "applied"))
    {
        return Err(StatusCode::CONFLICT);
    }
    let value = proposal["proposal"]["value"]
        .as_i64()
        .filter(|v| (10..=100).contains(v))
        .ok_or(StatusCode::BAD_GATEWAY)?;
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("INSERT INTO chat_room_policy(room_id) VALUES($1) ON CONFLICT DO NOTHING")
        .bind(&room)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let (before, revision): (i64, i64) = sqlx::query_as(
        "SELECT context_messages,revision FROM chat_room_policy WHERE room_id=$1 FOR UPDATE",
    )
    .bind(&room)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let existing:Option<(Uuid,bool)>=sqlx::query_as("SELECT apply_receipt,applied FROM chat_suggestion_receipts WHERE proposal_id=$1 AND room_id=$2").bind(id).bind(&room).fetch_optional(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    let receipt = if let Some((receipt, true)) = existing {
        receipt
    } else if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    } else {
        if proposal["status"] != "approved" {
            return Err(StatusCode::CONFLICT);
        }
        let receipt = Uuid::new_v4();
        sqlx::query(
            "UPDATE chat_room_policy SET context_messages=$2,revision=revision+1 WHERE room_id=$1",
        )
        .bind(&room)
        .bind(value)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        sqlx::query("INSERT INTO chat_suggestion_receipts(proposal_id,room_id,before_value,after_value,applied_revision,apply_receipt) VALUES($1,$2,$3,$4,$5,$6)").bind(id).bind(&room).bind(before).bind(value).bind(revision+1).bind(receipt).execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
        receipt
    };
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let ack = client
        .post(format!("{base}/api/v1/suggestions/{id}/receipt"))
        .bearer_auth(token)
        .json(&json!({"decision":"applied","receipt_id":receipt,"value":value}))
        .send()
        .await
        .is_ok_and(|r| r.status().is_success());
    Ok(Json(
        json!({"receipt_id":receipt,"value":value,"applied":true,"analytics_acknowledged":ack}),
    ))
}
pub(super) async fn rollback(
    State(state): State<StateData>,
    headers: HeaderMap,
    Path((room, id)): Path<(String, Uuid)>,
    Json(input): Json<Confirm>,
) -> Result<Json<Value>, StatusCode> {
    owner(&state, &headers, &room).await?;
    if !input.confirmed {
        return Err(StatusCode::BAD_REQUEST);
    }
    let (client, base, token) = remote()?;
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let (current, revision): (i64, i64) = sqlx::query_as(
        "SELECT context_messages,revision FROM chat_room_policy WHERE room_id=$1 FOR UPDATE",
    )
    .bind(&room)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::CONFLICT)?;
    let row:Option<(i64,i64,i64,bool,Option<Uuid>)>=sqlx::query_as("SELECT before_value,after_value,applied_revision,applied,rollback_receipt FROM chat_suggestion_receipts WHERE proposal_id=$1 AND room_id=$2 FOR UPDATE").bind(id).bind(&room).fetch_optional(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    let (before, after, expected, applied, old) = row.ok_or(StatusCode::NOT_FOUND)?;
    let receipt = if !applied {
        old.ok_or(StatusCode::CONFLICT)?
    } else {
        if current != after || revision != expected {
            return Err(StatusCode::CONFLICT);
        }
        let receipt = Uuid::new_v4();
        sqlx::query(
            "UPDATE chat_room_policy SET context_messages=$2,revision=revision+1 WHERE room_id=$1",
        )
        .bind(&room)
        .bind(before)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        sqlx::query("UPDATE chat_suggestion_receipts SET applied=false,rollback_receipt=$3 WHERE proposal_id=$1 AND room_id=$2").bind(id).bind(&room).bind(receipt).execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
        receipt
    };
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let ack = client
        .post(format!("{base}/api/v1/suggestions/{id}/receipt"))
        .bearer_auth(token)
        .json(&json!({"decision":"rolled_back","receipt_id":receipt,"value":before}))
        .send()
        .await
        .is_ok_and(|r| r.status().is_success());
    Ok(Json(
        json!({"receipt_id":receipt,"value":before,"rolled_back":true,"analytics_acknowledged":ack}),
    ))
}
