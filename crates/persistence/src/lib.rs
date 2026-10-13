//! 訊息 Repository 的 SQLite Adapter：訊息與 Outbox 同交易，租約 claim／CAS ACK／重試。
//! 對應參考持久化 Adapter 的職責，保留成品 durable realtime 與安全備份。
pub use chat_application as application;
pub mod database;
pub mod topics;
use event_contract::ProductEvent;
use sqlx::{Row, SqlitePool};
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatWriter {
    pool: SqlitePool,
}
impl application::MessageRepository for ChatWriter {
    fn commit<'a>(&'a self, input: application::MessageInput<'a>) -> application::CommitFuture<'a> {
        Box::pin(async move {
            match self
                .persist_with_id(
                    input.id,
                    input.room,
                    input.content,
                    input.author_kind,
                    input.author_id,
                )
                .await
            {
                Ok((id, inserted)) => Ok(application::Committed { id, inserted }),
                Err(sqlx::Error::Protocol(_)) => Err(application::MessageError::Conflict),
                Err(_) => Err(application::MessageError::Unavailable),
            }
        })
    }
}
impl ChatWriter {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 必須先 commit 才能呼叫使用者程式的 WS publish。
    pub async fn persist(&self, room_id: &str, content: &str) -> Result<Uuid, sqlx::Error> {
        self.persist_as(room_id, content, "anonymous_user", None)
            .await
    }

    pub async fn persist_as(
        &self,
        room_id: &str,
        content: &str,
        author_kind: &str,
        author_id: Option<&str>,
    ) -> Result<Uuid, sqlx::Error> {
        self.persist_with_id(Uuid::new_v4(), room_id, content, author_kind, author_id)
            .await
            .map(|(id, _)| id)
    }

    /// 重送 ID 僅能對應相同房間、作者與本文；
    /// INSERT 衝突序列化保證並發寫入只建立一筆 Outbox。
    pub async fn persist_with_id(
        &self,
        id: Uuid,
        room_id: &str,
        content: &str,
        author_kind: &str,
        author_id: Option<&str>,
    ) -> Result<(Uuid, bool), sqlx::Error> {
        if id.is_nil() {
            return Err(sqlx::Error::Protocol("invalid message id".into()));
        }
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let result = Self::stage(&mut tx, id, room_id, content, author_kind, author_id).await?;
        tx.commit().await?;
        Ok(result)
    }
    async fn stage(
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        id: Uuid,
        room_id: &str,
        content: &str,
        author_kind: &str,
        author_id: Option<&str>,
    ) -> Result<(Uuid, bool), sqlx::Error> {
        let event = ProductEvent::chat_message_persisted(id);
        let inserted = sqlx::query("INSERT INTO chat_messages(id, room_id, content, author_kind, author_id) VALUES (?1,?2,?3,?4,?5) ON CONFLICT (id) DO NOTHING")
            .bind(id)
            .bind(room_id)
            .bind(content)
            .bind(author_kind)
            .bind(author_id)
            .execute(&mut **tx)
            .await?.rows_affected() == 1;
        if !inserted {
            let matches: bool = sqlx::query_scalar("SELECT room_id=?2 AND content=?3 AND author_kind=?4 AND author_id IS ?5 FROM chat_messages WHERE id=?1")
                .bind(id).bind(room_id).bind(content).bind(author_kind).bind(author_id)
                .fetch_one(&mut **tx).await?;
            if !matches {
                return Err(sqlx::Error::Protocol("message id conflict".into()));
            }
            return Ok((id, false));
        }
        sqlx::query("INSERT INTO chat_outbox(event_id,message_id,schema_version,source_app,event_type,traceparent) VALUES (?1,?2,?3,?4,?5,?6)")
            .bind(event.event_id).bind(id).bind(i16::try_from(event.schema_version).unwrap())
            .bind(event.source_app).bind(event.event_type).bind(copilot_telemetry::outgoing_headers().get("traceparent")).execute(&mut **tx).await?;
        topics::stage(tx, id, room_id, author_kind, author_id, content).await?;
        sqlx::query("INSERT INTO chat_realtime(payload)VALUES(?1)")
            .bind(serde_json::json!({"type":"message","id":id,"room_id":room_id}).to_string())
            .execute(&mut **tx)
            .await?;
        Ok((id, true))
    }
    pub async fn persist_persona_turn(
        &self,
        room: &str,
        persona: Uuid,
        revision: i64,
        run: Uuid,
        lease: Uuid,
        turn: &serde_json::Value,
    ) -> Result<Uuid, sqlx::Error> {
        let answer = turn["answer"]
            .as_str()
            .filter(|s| !s.trim().is_empty() && s.len() <= 65536)
            .ok_or_else(|| sqlx::Error::Protocol("invalid persona answer".into()))?;
        let memory = turn["memory"]
            .as_str()
            .filter(|s| s.len() <= 2048)
            .ok_or_else(|| sqlx::Error::Protocol("invalid persona memory".into()))?;
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let valid:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM chat_ai_room_locks WHERE room_id=?1 AND lease_token=?2 AND expires_at>unixepoch('subsec'))").bind(room).bind(lease).fetch_one(&mut *tx).await?;
        if !valid {
            return Err(sqlx::Error::Protocol("persona lease expired".into()));
        }
        let id = Uuid::new_v4();
        let mut record = turn.clone();
        record["message_id"] = serde_json::json!(id);
        let updated=sqlx::query("UPDATE chat_persona_runs SET result=json_insert(result,'$[#]',json(?2)),updated_at=unixepoch('subsec') WHERE id=?1 AND room_id=?3 AND status='running' AND json_array_length(result)<max_turns AND reserved_tokens>=512*(json_array_length(result)+1)").bind(run).bind(&record).bind(room).execute(&mut *tx).await?;
        if updated.rows_affected() != 1 {
            return Err(sqlx::Error::Protocol("persona run stopped".into()));
        }
        let updated=sqlx::query("UPDATE chat_personas SET memory=?1,last_spoke_at=unixepoch('subsec') WHERE id=?2 AND room_id=?3 AND revision=?4 AND enabled").bind(memory).bind(persona).bind(room).bind(revision).execute(&mut *tx).await?;
        if updated.rows_affected() != 1 {
            return Err(sqlx::Error::Protocol("persona changed".into()));
        }
        Self::stage(&mut tx, id, room, answer, "ai", Some(&persona.to_string())).await?;
        tx.commit().await?;
        Ok(id)
    }
}

#[derive(Debug, Clone)]
pub struct LeasedEvent {
    pub event: ProductEvent,
    pub lease_token: Uuid,
    pub attempts: i32,
    pub traceparent: Option<String>,
}

#[derive(Clone)]
pub struct OutboxStore {
    pool: SqlitePool,
}
impl OutboxStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// atomic SQLite claim 防止多 worker 併發領取同一事件，租約過期允許重新領取。
    pub async fn claim_one(&self) -> Result<Option<LeasedEvent>, sqlx::Error> {
        let token = Uuid::new_v4();
        let row = sqlx::query(
            "UPDATE chat_outbox SET status='leased',lease_token=?1,lease_until=unixepoch('subsec')+60,attempts=attempts+1 WHERE event_id=(SELECT event_id FROM chat_outbox WHERE (status='pending' AND next_attempt_at<=unixepoch('subsec')) OR (status='leased' AND lease_until<=unixepoch('subsec')) ORDER BY created_at,event_id LIMIT 1) RETURNING event_id,schema_version,source_app,event_type,attempts,traceparent",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| LeasedEvent {
            event: ProductEvent {
                schema_version: r.get::<i16, _>("schema_version") as u16,
                event_id: r.get("event_id"),
                source_app: r.get("source_app"),
                event_type: r.get("event_type"),
            },
            lease_token: token,
            attempts: r.get("attempts"),
            traceparent: r.get("traceparent"),
        }))
    }

    pub async fn ack(&self, lease: &LeasedEvent) -> Result<bool, sqlx::Error> {
        let done = sqlx::query(
            "UPDATE chat_outbox SET status='delivered', delivered_at=unixepoch('subsec'),
             lease_token=NULL, lease_until=NULL
             WHERE event_id=?1 AND lease_token=?2 AND status='leased' AND lease_until>unixepoch('subsec')",
        )
        .bind(lease.event.event_id)
        .bind(lease.lease_token)
        .execute(&self.pool)
        .await?;
        Ok(done.rows_affected() == 1)
    }

    /// 拒絕過期 token 的 ACK/重試；超過 10 次封鎖供人工處置。
    pub async fn fail(
        &self,
        lease: &LeasedEvent,
        error_code: &str,
        retryable: bool,
    ) -> Result<bool, sqlx::Error> {
        let blocked = !retryable || lease.attempts >= 10;
        let delay_seconds = (1_i64 << lease.attempts.min(8)).min(300);
        let result = sqlx::query(
            "UPDATE chat_outbox
             SET status=CASE WHEN ?3 THEN 'blocked' ELSE 'pending' END,
                 next_attempt_at=unixepoch('subsec')+?4,
                 lease_token=NULL, lease_until=NULL, last_error=?5
             WHERE event_id=?1 AND lease_token=?2 AND status='leased' AND lease_until>unixepoch('subsec')",
        )
        .bind(lease.event.event_id)
        .bind(lease.lease_token)
        .bind(blocked)
        .bind(delay_seconds)
        .bind(error_code)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() == 1)
    }
}

pub async fn dispatch_once(
    store: &OutboxStore,
    _client: &reqwest::Client,
    endpoint: &str,
    token: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let Some(lease) = store.claim_one().await? else {
        return Ok(false);
    };
    let parent = lease.traceparent.clone().or_else(|| {
        copilot_telemetry::outgoing_headers()
            .get("traceparent")
            .cloned()
    });
    match analytics_transport::deliver(endpoint, token, &lease.event, parent.as_deref()).await {
        Ok(()) => {
            store.ack(&lease).await?;
        }
        Err(status) => {
            store
                .fail(
                    &lease,
                    "grpc_delivery_failed",
                    analytics_transport::retryable(&status),
                )
                .await?;
        }
    }
    Ok(true)
}

/// 預期外部配置有權限的 HTTPS endpoint 與 Bearer Token。
pub async fn run_dispatcher(
    pool: SqlitePool,
    endpoint: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let store = OutboxStore::new(pool);
    loop {
        // 暫時性 DB 故障不能永久終止投遞背景工作；不輸出敏感錯誤內容。
        let privacy = topics::dispatch_withdrawal(&store.pool, &client, endpoint, token).await;
        match dispatch_once(&store, &client, endpoint, token).await {
            Ok(false) if matches!(privacy, Ok(true)) => tokio::task::yield_now().await,
            Ok(true) => tokio::task::yield_now().await,
            Ok(false) => tokio::time::sleep(Duration::from_millis(500)).await,
            Err(_) => {
                eprintln!("outbox_dispatch_failed");
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests;
