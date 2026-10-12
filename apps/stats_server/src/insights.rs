//! Immutable aggregate evidence, bounded numeric suggestions and explicit review/receipt transitions.
use super::{AnalyticsState, authorized};
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
pub(super) struct Generate {
    pub target_app: String,
}
fn schema() -> Value {
    json!({"type":"object","properties":{"fact_indices":{"type":"array","items":{"type":"integer"},"minItems":1,"maxItems":3},"action":{"type":"string","enum":["chat_context_messages","pet_review_min_days"]},"value":{"type":"integer"}},"required":["fact_indices","action","value"],"additionalProperties":false})
}
fn facts(snapshot: &Value) -> Option<Vec<String>> {
    let rows = snapshot["rows"].as_array()?;
    let total = rows
        .iter()
        .try_fold(0_i64, |sum, row| sum.checked_add(row["total"].as_i64()?))?;
    let mut facts = vec![format!(
        "最近七天共記錄 {total} 筆事件，事件次數不代表人數、學習品質或因果。"
    )];
    for row in rows {
        let count = row["total"].as_i64()?;
        if count < 0 {
            return None;
        }
        let date = row["day"].as_str()?;
        let event = row["event_type"].as_str()?;
        facts.push(format!("台北日期 {date} 的 {event} 共記錄 {count} 次。"));
    }
    Some(facts)
}
fn grounded_proposal(snapshot: &Value, selection: &Value) -> Option<Value> {
    let candidates = facts(snapshot)?;
    let indices = selection["fact_indices"].as_array()?;
    if !(1..=3).contains(&indices.len()) {
        return None;
    }
    let mut used = std::collections::HashSet::new();
    let mut summary = String::new();
    for index in indices {
        let i = usize::try_from(index.as_u64()?).ok()?;
        if !used.insert(i) {
            return None;
        }
        summary.push_str(candidates.get(i)?);
    }
    let proposal = json!({"summary":summary,"action":selection["action"],"value":selection["value"],"rationale":"AI 選擇以上實際事件數作為觀察線索；設定建議是待人工審核的實驗，事件數不能證明效果或品質。","grounding":{"fact_indices":indices,"facts":candidates,"method":"verified_snapshot_fact_selection"}});
    valid(snapshot["target_app"].as_str()?, &proposal).then_some(proposal)
}
fn valid(target: &str, proposal: &Value) -> bool {
    let Some(value) = proposal["value"].as_i64() else {
        return false;
    };
    let Some(summary) = proposal["summary"].as_str() else {
        return false;
    };
    let sentences = summary
        .chars()
        .filter(|c| matches!(c, '。' | '！' | '？' | '!' | '?'))
        .count();
    (1..=3).contains(&sentences)
        && summary.len() <= 2000
        && proposal["rationale"]
            .as_str()
            .is_some_and(|s| !s.trim().is_empty() && s.len() <= 2000)
        && match (target, proposal["action"].as_str()) {
            ("web_chat", Some("chat_context_messages")) => (10..=100).contains(&value),
            ("desktop_pet", Some("pet_review_min_days")) => (1..=30).contains(&value),
            _ => false,
        }
}
pub(super) async fn generate(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Json(input): Json<Generate>,
) -> Result<Json<Value>, StatusCode> {
    if !authorized(&headers, &state.admin_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if !matches!(input.target_app.as_str(), "web_chat" | "desktop_pet") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let _permit = state
        .ai_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;
    let rows:Vec<(String,String,i64)>=sqlx::query_as("SELECT (received_at AT TIME ZONE 'Asia/Taipei')::date::text,event_type,count(*)::bigint FROM product_events WHERE NOT archived AND NOT consent_withdrawn AND source_app=$1 AND received_at>=now()-interval '7 days' GROUP BY 1,2 ORDER BY 1,2").bind(&input.target_app).fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    let snapshot = json!({"target_app":input.target_app,"window":"last_7_days","timezone":"Asia/Taipei","rows":rows.into_iter().map(|(day,event_type,total)|json!({"day":day,"event_type":event_type,"total":total})).collect::<Vec<_>>()});
    let candidates = facts(&snapshot).ok_or(StatusCode::BAD_GATEWAY)?;
    let instructions = "根據真實事件聚合快照選出一至三個值得觀察的 facts 索引（從0開始、不重複）。不要生成新的事實或數字。只提出 target 對應的安全設定實驗：web_chat 用 chat_context_messages:10..100，desktop_pet 用 pet_review_min_days:1..30。提案必須經人工審核、可回退；事件數不代表人數、品質或因果。";
    let selection = openai_adapter::agent::structured(
        json!({"snapshot":snapshot,"facts":candidates}),
        instructions,
        schema(),
    )
    .await
    .map_err(|_| StatusCode::BAD_GATEWAY)?;
    let proposal = grounded_proposal(&snapshot, &selection).ok_or(StatusCode::BAD_GATEWAY)?;
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO analysis_suggestions(id,target_app,metric_snapshot,summary,proposal) VALUES ($1,$2,$3,$4,$5)").bind(id).bind(&input.target_app).bind(&snapshot).bind(proposal["summary"].as_str().unwrap()).bind(&proposal).execute(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(
        json!({"id":id,"snapshot":snapshot,"proposal":proposal,"status":"pending"}),
    ))
}
fn source(headers: &HeaderMap, state: &AnalyticsState) -> Option<&'static str> {
    if authorized(headers, &state.chat_token) {
        Some("web_chat")
    } else if authorized(headers, &state.pet_token) {
        Some("desktop_pet")
    } else {
        None
    }
}
pub(super) async fn list(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Value>>, StatusCode> {
    if !authorized(&headers, &state.api_token) && !authorized(&headers, &state.admin_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let rows:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',id,'target_app',target_app,'snapshot',metric_snapshot,'summary',summary,'proposal',proposal,'status',status,'receipt_id',receipt_id,'evaluation',evaluation) FROM analysis_suggestions ORDER BY created_at DESC LIMIT 100").fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(rows))
}
pub(super) async fn get(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let app = source(&headers, &state).ok_or(StatusCode::UNAUTHORIZED)?;
    let row:Option<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',id,'target_app',target_app,'snapshot',metric_snapshot,'proposal',proposal,'status',status) FROM analysis_suggestions WHERE id=$1 AND target_app=$2").bind(id).bind(app).fetch_optional(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    row.map(Json).ok_or(StatusCode::NOT_FOUND)
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Review {
    decision: String,
    confirmed: bool,
}
pub(super) async fn review(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(input): Json<Review>,
) -> Result<StatusCode, StatusCode> {
    if !authorized(&headers, &state.admin_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if !input.confirmed || !matches!(input.decision.as_str(), "approved" | "rejected") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let row=sqlx::query("UPDATE analysis_suggestions SET status=$2,updated_at=now() WHERE id=$1 AND status='pending'").bind(id).bind(input.decision).execute(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    if row.rows_affected() != 1 {
        return Err(StatusCode::CONFLICT);
    }
    Ok(StatusCode::NO_CONTENT)
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Receipt {
    pub decision: String,
    pub receipt_id: Uuid,
    pub value: i64,
}
pub(super) async fn receipt(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(input): Json<Receipt>,
) -> Result<StatusCode, StatusCode> {
    let app = source(&headers, &state).ok_or(StatusCode::UNAUTHORIZED)?;
    if !matches!(input.decision.as_str(), "applied" | "rolled_back") || input.receipt_id.is_nil() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let expected:Option<i64>=sqlx::query_scalar("SELECT (proposal->>'value')::bigint FROM analysis_suggestions WHERE id=$1 AND target_app=$2").bind(id).bind(app).fetch_optional(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    let expected = expected.ok_or(StatusCode::NOT_FOUND)?;
    if (input.decision == "applied" && input.value != expected)
        || (app == "web_chat" && !(10..=100).contains(&input.value))
        || (app == "desktop_pet" && !(1..=30).contains(&input.value))
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let from = if input.decision == "applied" {
        "approved"
    } else {
        "applied"
    };
    let row=sqlx::query("UPDATE analysis_suggestions SET status=$3,receipt_id=$4,updated_at=now() WHERE id=$1 AND target_app=$2 AND status=$5").bind(id).bind(app).bind(&input.decision).bind(input.receipt_id).bind(from).execute(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    if row.rows_affected() == 0 {
        let same:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM analysis_suggestions WHERE id=$1 AND target_app=$2 AND status=$3 AND receipt_id=$4)").bind(id).bind(app).bind(input.decision).bind(input.receipt_id).fetch_one(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
        if !same {
            return Err(StatusCode::CONFLICT);
        }
    }
    Ok(StatusCode::NO_CONTENT)
}
/// Descriptive comparison only: event totals do not establish learning quality or causality.
pub(super) async fn evaluate(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(input): Json<Evaluate>,
) -> Result<Json<Value>, StatusCode> {
    if !authorized(&headers, &state.admin_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if !input.confirmed {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let row: Option<(String, Value, String)> = sqlx::query_as(
        "SELECT target_app,metric_snapshot,status FROM analysis_suggestions WHERE id=$1 FOR UPDATE",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let (target, before, status) = row.ok_or(StatusCode::NOT_FOUND)?;
    if !matches!(status.as_str(), "applied" | "rolled_back") {
        return Err(StatusCode::CONFLICT);
    }
    let rows:Vec<(String,String,i64)>=sqlx::query_as("SELECT (received_at AT TIME ZONE 'Asia/Taipei')::date::text,event_type,count(*)::bigint FROM product_events WHERE NOT archived AND NOT consent_withdrawn AND source_app=$1 AND received_at>=now()-interval '7 days' GROUP BY 1,2 ORDER BY 1,2")
        .bind(&target).fetch_all(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    let after = json!({"target_app":target,"window":"last_7_days","timezone":"Asia/Taipei","rows":rows.into_iter().map(|(day,event_type,total)|json!({"day":day,"event_type":event_type,"total":total})).collect::<Vec<_>>()});
    let evaluation = json!({"before":before,"after":after,"interpretation":"Descriptive event counts only; overlapping windows and changing exposure cannot prove causal impact, learning quality, or unique users."});
    sqlx::query("UPDATE analysis_suggestions SET evaluation=$2,updated_at=now() WHERE id=$1")
        .bind(id)
        .bind(&evaluation)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(evaluation))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Evaluate {
    confirmed: bool,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_numbers_and_claims_come_only_from_verified_snapshot_facts() {
        let snapshot = json!({"target_app":"web_chat","rows":[{"day":"2026-10-12","event_type":"message.persisted","total":3}]});
        let choice = json!({"fact_indices":[0,1],"action":"chat_context_messages","value":30});
        let proposal = grounded_proposal(&snapshot, &choice).unwrap();
        assert!(proposal["summary"].as_str().unwrap().contains("3 筆"));
        for indices in [
            json!([99]),
            json!([-1]),
            json!([0, 0]),
            json!([]),
            json!([0, 1, 0, 1]),
        ] {
            let mut bad = choice.clone();
            bad["fact_indices"] = indices;
            assert!(grounded_proposal(&snapshot, &bad).is_none());
        }
        let mut bad = choice;
        bad["action"] = json!("pet_review_min_days");
        assert!(grounded_proposal(&snapshot, &bad).is_none());
    }
    #[test]
    fn proposal_is_bounded_and_cannot_change_target() {
        let p = json!({"summary":"目前有三筆事件。","action":"chat_context_messages","value":30,"rationale":"依三筆事件提出建議"});
        assert!(valid("web_chat", &p));
        assert!(!valid("desktop_pet", &p));
        let mut p = p;
        p["value"] = json!(10000);
        assert!(!valid("web_chat", &p));
    }
}
