//! SQLite consent and reliable withdrawal Adapter; only fixed vocabulary leaves Chat.
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
pub async fn consent(
    pool: &SqlitePool,
    user: Uuid,
    room: &str,
) -> Result<serde_json::Value, sqlx::Error> {
    let enabled:bool=sqlx::query_scalar("SELECT COALESCE((SELECT enabled FROM chat_topic_consent WHERE user_id=?1 AND room_id=?2),false)").bind(user).bind(room).fetch_one(pool).await?;
    let pending:i64=sqlx::query_scalar("SELECT count(*) FROM chat_topic_events e JOIN chat_topic_consent c USING(user_id,room_id) WHERE e.user_id=?1 AND e.room_id=?2 AND e.created_at<=c.withdraw_before AND NOT e.withdrawal_ack").bind(user).bind(room).fetch_one(pool).await?;
    Ok(
        serde_json::json!({"enabled":enabled,"withdrawal_pending":pending,"vocabulary":event_contract::topics::VOCABULARY}),
    )
}
pub async fn set_consent(
    pool: &SqlitePool,
    user: Uuid,
    room: &str,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO chat_topic_consent(user_id,room_id,enabled,withdraw_before,withdraw_pending)VALUES(?1,?2,?3,CASE WHEN ?3 THEN NULL ELSE unixepoch('subsec') END,NOT ?3) ON CONFLICT(user_id,room_id)DO UPDATE SET enabled=?3,withdraw_before=CASE WHEN ?3 THEN chat_topic_consent.withdraw_before ELSE unixepoch('subsec') END,withdraw_pending=chat_topic_consent.withdraw_pending OR NOT ?3").bind(user).bind(room).bind(enabled).execute(pool).await?;
    Ok(())
}
pub(crate) async fn stage(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    message: Uuid,
    room: &str,
    author_kind: &str,
    author_id: Option<&str>,
    content: &str,
) -> Result<(), sqlx::Error> {
    if author_kind != "user" {
        return Ok(());
    }
    let Some(user) = author_id.and_then(|id| Uuid::parse_str(id).ok()) else {
        return Ok(());
    };
    let enabled: Option<bool> = sqlx::query_scalar(
        "SELECT enabled FROM chat_topic_consent WHERE user_id=?1 AND room_id=?2",
    )
    .bind(user)
    .bind(room)
    .fetch_optional(&mut **tx)
    .await?;
    if enabled != Some(true) {
        return Ok(());
    }
    for word in event_contract::topics::occurrences(content) {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO chat_topic_events(event_id,message_id,user_id,room_id)VALUES(?1,?2,?3,?4)",
        )
        .bind(id)
        .bind(message)
        .bind(user)
        .bind(room)
        .execute(&mut **tx)
        .await?;
        sqlx::query("INSERT INTO chat_outbox(event_id,schema_version,source_app,event_type,traceparent)VALUES(?1,1,'web_chat',?2,?3)").bind(id).bind(format!("keyword.{word}")).bind(copilot_telemetry::outgoing_headers().get("traceparent")).execute(&mut **tx).await?;
    }
    Ok(())
}
async fn queue_batch(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    let row=sqlx::query("SELECT user_id,room_id,withdraw_before AS cutoff FROM chat_topic_consent WHERE withdraw_pending ORDER BY user_id,room_id LIMIT 1").fetch_optional(&mut *tx).await?;
    if let Some(row) = row {
        let user: Uuid = row.get("user_id");
        let room: String = row.get("room_id");
        let cutoff: f64 = row.get("cutoff");
        let ids:Vec<Uuid>=sqlx::query_scalar("SELECT event_id FROM chat_topic_events WHERE user_id=?1 AND room_id=?2 AND created_at<=?3 AND NOT withdrawal_queued ORDER BY created_at,event_id LIMIT 500").bind(user).bind(&room).bind(cutoff).fetch_all(&mut *tx).await?;
        if ids.is_empty() {
            sqlx::query("UPDATE chat_topic_consent SET withdraw_pending=false WHERE user_id=?1 AND room_id=?2").bind(user).bind(room).execute(&mut *tx).await?;
        } else {
            sqlx::query("INSERT INTO chat_topic_withdraw_outbox(id,event_ids)VALUES(?1,?2)")
                .bind(Uuid::new_v4())
                .bind(
                    serde_json::to_string(&ids)
                        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?,
                )
                .execute(&mut *tx)
                .await?;
            for event_id in ids {
                sqlx::query(
                    "UPDATE chat_topic_events SET withdrawal_queued=true WHERE event_id=?1",
                )
                .bind(event_id)
                .execute(&mut *tx)
                .await?;
            }
        }
    }
    tx.commit().await?;
    Ok(())
}
pub async fn dispatch_withdrawal(
    pool: &SqlitePool,
    client: &reqwest::Client,
    endpoint: &str,
    token: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    queue_batch(pool).await?;
    let lease = Uuid::new_v4();
    let row=sqlx::query("UPDATE chat_topic_withdraw_outbox SET status='leased',lease_token=?1,lease_until=unixepoch('subsec')+30,attempts=attempts+1 WHERE id=(SELECT id FROM chat_topic_withdraw_outbox WHERE (status='pending' AND next_attempt_at<=unixepoch('subsec')) OR (status='leased' AND lease_until<=unixepoch('subsec')) ORDER BY id LIMIT 1) RETURNING id,event_ids,attempts").bind(lease).fetch_optional(pool).await?;
    let Some(row) = row else {
        return Ok(false);
    };
    let id: Uuid = row.get("id");
    let ids: Vec<Uuid> = serde_json::from_str(row.get::<&str, _>("event_ids"))?;
    let attempts: i32 = row.get("attempts");
    let mut url = reqwest::Url::parse(endpoint)?;
    url.set_path("/api/v1/topics/withdraw");
    url.set_query(None);
    url.set_fragment(None);
    let mut request = client
        .post(url)
        .bearer_auth(token)
        .json(&serde_json::json!({"event_ids":ids}));
    for (key, value) in copilot_telemetry::outgoing_headers() {
        request = request.header(key, value);
    }
    if request.send().await.is_ok_and(|r| r.status().is_success()) {
        let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
        let ack=sqlx::query("UPDATE chat_topic_withdraw_outbox SET status='delivered',lease_token=NULL,lease_until=NULL WHERE id=?1 AND lease_token=?2 AND status='leased' AND lease_until>unixepoch('subsec')").bind(id).bind(lease).execute(&mut *tx).await?;
        if ack.rows_affected() == 1 {
            for event_id in ids {
                sqlx::query("UPDATE chat_topic_events SET withdrawal_ack=true WHERE event_id=?1")
                    .bind(event_id)
                    .execute(&mut *tx)
                    .await?;
            }
        }
        tx.commit().await?;
    } else {
        let delay = (1_i64 << attempts.min(8)).min(300);
        sqlx::query("UPDATE chat_topic_withdraw_outbox SET status='pending',lease_token=NULL,lease_until=NULL,next_attempt_at=unixepoch('subsec')+?3 WHERE id=?1 AND lease_token=?2 AND status='leased' AND lease_until>unixepoch('subsec')").bind(id).bind(lease).bind(delay).execute(pool).await?;
    }
    Ok(true)
}
