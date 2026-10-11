//! Day 9 Chat：交易式 Outbox 的獨立對照實作。
//! 原始聊天室的 WebSocket 與 Domain 契約維持不變；本模組僅示範持久化邊界。
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthorKind { AnonymousUser, User, Ai, System }

impl AuthorKind {
    fn as_db(self) -> &'static str {
        match self { Self::AnonymousUser => "anonymous_user", Self::User => "user",
            Self::Ai => "ai", Self::System => "system" }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: Uuid,
    pub room_id: String,
    pub author: AuthorKind,
    pub author_id: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductEvent {
    pub schema_version: u16,
    pub event_id: Uuid,
    pub source_app: String,
    pub event_type: String,
}
impl ProductEvent {
    pub fn persisted(id: Uuid) -> Self {
        Self { schema_version: 1, event_id: id, source_app: "web_chat".into(),
            event_type: "message.persisted".into() }
    }
}

/// Port：應用層只看這個介面；UUID/VARCHAR 的 SQLx 細節放在 Adapter。
#[async_trait::async_trait]
pub trait MessageWritePort: Send + Sync {
    async fn save_with_outbox(&self, message: &ChatMessage) -> Result<ProductEvent, sqlx::Error>;
}

#[derive(Clone)]
pub struct PgChatWriter { pool: PgPool }
impl PgChatWriter {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait::async_trait]
impl MessageWritePort for PgChatWriter {
    async fn save_with_outbox(&self, message: &ChatMessage) -> Result<ProductEvent, sqlx::Error> {
        let event = ProductEvent::persisted(message.id);
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "INSERT INTO chat_messages (id, room_id, author_kind, author_id, content)
             VALUES ($1, $2, $3, $4, $5)"
        ).bind(message.id).bind(&message.room_id).bind(message.author.as_db())
          .bind(&message.author_id).bind(&message.content)
          .execute(&mut *tx).await?;
        sqlx::query(
            "INSERT INTO chat_outbox (event_id, message_id, schema_version, source_app, event_type)
             VALUES ($1, $2, $3, $4, $5)"
        ).bind(event.event_id).bind(message.id).bind(i32::from(event.schema_version))
          .bind(&event.source_app).bind(&event.event_type)
          .execute(&mut *tx).await?;
        // 只有 commit 成功，呼叫端才可以執行 WS 廣播。
        tx.commit().await?;
        Ok(event)
    }
}

#[derive(Debug, Clone)]
pub struct LeasedEvent {
    pub event: ProductEvent,
    pub lease_id: Uuid,
    pub attempts: i32,
}

#[derive(Clone)]
pub struct PgOutbox { pool: PgPool }
impl PgOutbox {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    /// 單筆領取：SKIP LOCKED 可供多個 worker 競爭，同時避免批量 lease 過期。
    pub async fn claim(&self, lease_seconds: i32) -> Result<Option<LeasedEvent>, sqlx::Error> {
        let seconds = lease_seconds.max(1);
        let row = sqlx::query_as::<_, (Uuid, i32, String, String, Uuid, i32)>(
            "WITH due AS (
                SELECT event_id FROM chat_outbox
                WHERE (status = 'pending' AND next_attempt_at <= now())
                   OR (status = 'leased' AND leased_until <= now())
                ORDER BY next_attempt_at, created_at
                FOR UPDATE SKIP LOCKED
                LIMIT 1
             )
             UPDATE chat_outbox o
             SET status='leased', attempts=o.attempts+1, lease_id=$1,
                 leased_until=now()+($2::int * interval '1 second')
             FROM due WHERE o.event_id=due.event_id
             RETURNING o.event_id,o.schema_version,o.source_app,o.event_type,o.lease_id,o.attempts"
        ).bind(Uuid::new_v4()).bind(seconds).fetch_optional(&self.pool).await?;
        Ok(row.map(|(event_id, version, source_app, event_type, lease_id, attempts)| LeasedEvent {
            event: ProductEvent { schema_version: version as u16, event_id, source_app, event_type },
            lease_id, attempts
        }))
    }

    /// Fence：過期 worker 不得 ACK 新 lease。
    pub async fn acknowledge(&self, lease: &LeasedEvent) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE chat_outbox SET status='delivered', delivered_at=now(),
                lease_id=NULL, leased_until=NULL
             WHERE event_id=$1 AND lease_id=$2 AND status='leased' AND leased_until>now()"
        ).bind(lease.event.event_id).bind(lease.lease_id).execute(&self.pool).await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn fail(&self, lease: &LeasedEvent, retryable: bool) -> Result<bool, sqlx::Error> {
        let stop = !retryable || lease.attempts >= 10;
        let delay = (2_i64.pow((lease.attempts as u32).min(8))).min(300);
        let result = sqlx::query(
            "UPDATE chat_outbox
             SET status=CASE WHEN $3 THEN 'blocked' ELSE 'pending' END,
                 next_attempt_at=now()+($4::bigint * interval '1 second'),
                 lease_id=NULL, leased_until=NULL
             WHERE event_id=$1 AND lease_id=$2 AND status='leased' AND leased_until>now()"
        ).bind(lease.event.event_id).bind(lease.lease_id).bind(stop)
          .bind(delay).execute(&self.pool).await?;
        Ok(result.rows_affected()==1)
    }
}

/// 解析投遞結果：只有 2xx 可 ACK；不將 4xx 誤判為可無限重試。
pub fn retryable_status(status: u16) -> bool {
    status == 408 || status == 425 || status == 429 || (500..600).contains(&status)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn event_contains_four_fields_only() {
        let id=Uuid::new_v4();
        let event=ProductEvent::persisted(id);
        let json=serde_json::to_value(event).unwrap();
        assert_eq!(json.as_object().unwrap().len(),4);
        assert_eq!(json["event_id"], id.to_string());
        assert_eq!(json["source_app"],"web_chat");
    }
    #[test]
    fn retry_contract() {
        for status in [408,425,429,500,503] { assert!(retryable_status(status)); }
        for status in [200,201,400,401,403,404,409,422] { assert!(!retryable_status(status)); }
    }
}
