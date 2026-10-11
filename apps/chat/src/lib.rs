//! Day 9: 訊息與 Outbox 同交易、租約 claim、CAS ACK、可重試投遞。
use event_contract::ProductEvent;
use sqlx::{PgPool, Row};
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatWriter {
    pool: PgPool,
}
impl ChatWriter {
    pub fn new(pool: PgPool) -> Self {
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
        let id = Uuid::new_v4();
        let event = ProductEvent::chat_message_persisted(id);
        let mut tx = self.pool.begin().await?;
        sqlx::query("INSERT INTO chat_messages(id, room_id, content, author_kind, author_id) VALUES ($1,$2,$3,$4,$5)")
            .bind(id)
            .bind(room_id)
            .bind(content)
            .bind(author_kind)
            .bind(author_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO chat_outbox(event_id,message_id,schema_version,source_app,event_type,traceparent) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(event.event_id).bind(id).bind(i16::try_from(event.schema_version).unwrap())
            .bind(event.source_app).bind(event.event_type).bind(copilot_telemetry::outgoing_headers().get("traceparent")).execute(&mut *tx).await?;
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
    pool: PgPool,
}
impl OutboxStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// SKIP LOCKED 防止多 worker 併發領取同一事件，租約過期允許重新領取。
    pub async fn claim_one(&self) -> Result<Option<LeasedEvent>, sqlx::Error> {
        let token = Uuid::new_v4();
        let row = sqlx::query(
            "WITH candidate AS (
               SELECT event_id FROM chat_outbox
               WHERE (status = 'pending' AND next_attempt_at <= now())
                  OR (status = 'leased' AND lease_until <= now())
               ORDER BY created_at, event_id FOR UPDATE SKIP LOCKED LIMIT 1
             )
             UPDATE chat_outbox AS e
             SET status='leased', lease_token=$1, lease_until=now()+interval '60 seconds',
                 attempts=e.attempts+1
             FROM candidate WHERE e.event_id=candidate.event_id
             RETURNING e.event_id, e.schema_version, e.source_app, e.event_type, e.attempts, e.traceparent",
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
            "UPDATE chat_outbox SET status='delivered', delivered_at=now(),
             lease_token=NULL, lease_until=NULL
             WHERE event_id=$1 AND lease_token=$2 AND status='leased' AND lease_until>now()",
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
             SET status=CASE WHEN $3 THEN 'blocked' ELSE 'pending' END,
                 next_attempt_at=now()+($4::bigint * interval '1 second'),
                 lease_token=NULL, lease_until=NULL, last_error=$5
             WHERE event_id=$1 AND lease_token=$2 AND status='leased' AND lease_until>now()",
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
    client: &reqwest::Client,
    endpoint: &str,
    token: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let Some(lease) = store.claim_one().await? else {
        return Ok(false);
    };
    let mut request = client.post(endpoint);
    for (key, value) in copilot_telemetry::outgoing_headers() {
        request = request.header(key, value);
    }
    if let Some(parent) = &lease.traceparent {
        request = request.header("traceparent", parent);
    }
    let result = request.bearer_auth(token).json(&lease.event).send().await;
    match result {
        Ok(response) if response.status().is_success() => {
            store.ack(&lease).await?;
        }
        Ok(response) => {
            let status = response.status();
            let retry = status.is_server_error() || matches!(status.as_u16(), 408 | 425 | 429);
            store
                .fail(&lease, &format!("http_{}", status.as_u16()), retry)
                .await?;
        }
        Err(_) => {
            store.fail(&lease, "transport_error", true).await?;
        }
    }
    Ok(true)
}

/// 預期外部配置有權限的 HTTPS endpoint 與 Bearer Token。
pub async fn run_dispatcher(
    pool: PgPool,
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
        if dispatch_once(&store, &client, endpoint, token)
            .await
            .is_err()
        {
            eprintln!("outbox_dispatch_failed");
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn event_id_is_message_id_and_stable_for_retries() {
        let id = Uuid::new_v4();
        assert_eq!(ProductEvent::chat_message_persisted(id).event_id, id);
    }
    #[sqlx::test(migrations = "./migrations")]
    async fn message_insert_has_exactly_one_outbox_event(pool: PgPool) {
        sqlx::query("INSERT INTO chat_rooms(id,name) VALUES ('room-1','Test')")
            .execute(&pool)
            .await
            .unwrap();
        let id = ChatWriter::new(pool.clone())
            .persist("room-1", "private")
            .await
            .unwrap();
        let row: (i64,) = sqlx::query_as("SELECT count(*) FROM chat_outbox WHERE event_id=$1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, 1);
    }
    #[sqlx::test(migrations = "./migrations")]
    async fn missing_room_rolls_back_both_message_and_event(pool: PgPool) {
        let result = ChatWriter::new(pool.clone())
            .persist("missing-room", "hello")
            .await;
        assert!(result.is_err(), "FK 失敗應回滾整筆交易");
        let row: (i64,) = sqlx::query_as("SELECT count(*) FROM chat_outbox")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, 0, "失敗不可留下 Outbox 事件");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn stale_lease_cannot_ack(pool: PgPool) {
        sqlx::query("INSERT INTO chat_rooms(id,name) VALUES ('room','Test')")
            .execute(&pool)
            .await
            .unwrap();
        let id = ChatWriter::new(pool.clone())
            .persist("room", "hello")
            .await
            .unwrap();
        let store = OutboxStore::new(pool.clone());
        let stale = store.claim_one().await.unwrap().unwrap();
        assert_eq!(stale.event.event_id, id);
        sqlx::query(
            "UPDATE chat_outbox SET lease_until=now()-interval '1 second' WHERE event_id=$1",
        )
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
        let fresh = store.claim_one().await.unwrap().unwrap();
        assert!(!store.ack(&stale).await.unwrap());
        assert!(store.ack(&fresh).await.unwrap());
    }
    #[sqlx::test(migrations = "./migrations")]
    async fn outbox_insert_failure_rolls_back_inserted_message(pool: PgPool) {
        sqlx::query("INSERT INTO chat_rooms(id,name) VALUES ('rollback','Test')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::raw_sql("CREATE FUNCTION reject_outbox() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'reject'; END $$; CREATE TRIGGER reject_outbox BEFORE INSERT ON chat_outbox FOR EACH ROW EXECUTE FUNCTION reject_outbox();")
            .execute(&pool).await.unwrap();
        assert!(
            ChatWriter::new(pool.clone())
                .persist("rollback", "not committed")
                .await
                .is_err()
        );
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM chat_messages")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 0);
    }
    #[sqlx::test(migrations = "./migrations")]
    async fn expired_lease_can_be_reclaimed_and_old_token_cannot_mutate(pool: PgPool) {
        sqlx::query("INSERT INTO chat_rooms(id,name) VALUES ('reclaim','Test')")
            .execute(&pool)
            .await
            .unwrap();
        ChatWriter::new(pool.clone())
            .persist("reclaim", "queued")
            .await
            .unwrap();
        let store = OutboxStore::new(pool.clone());
        let old = store.claim_one().await.unwrap().unwrap();
        sqlx::query("UPDATE chat_outbox SET lease_until=now()-interval '1 second'")
            .execute(&pool)
            .await
            .unwrap();
        let new = store.claim_one().await.unwrap().unwrap();
        assert_eq!(old.event.event_id, new.event.event_id);
        assert_ne!(old.lease_token, new.lease_token);
        assert!(!store.fail(&old, "stale", true).await.unwrap());
        assert!(!store.ack(&old).await.unwrap());
        assert!(store.ack(&new).await.unwrap());
    }
    #[sqlx::test(migrations = "./migrations")]
    async fn permanent_delivery_failure_is_blocked(pool: PgPool) {
        sqlx::query("INSERT INTO chat_rooms(id,name) VALUES ('blocked','Test')")
            .execute(&pool)
            .await
            .unwrap();
        ChatWriter::new(pool.clone())
            .persist("blocked", "queued")
            .await
            .unwrap();
        let store = OutboxStore::new(pool.clone());
        let lease = store.claim_one().await.unwrap().unwrap();
        assert!(store.fail(&lease, "http_401", false).await.unwrap());
        assert!(store.claim_one().await.unwrap().is_none());
    }
}
