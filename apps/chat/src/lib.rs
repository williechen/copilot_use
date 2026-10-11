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
        self.persist_with_id(Uuid::new_v4(), room_id, content, author_kind, author_id)
            .await
            .map(|(id, _)| id)
    }

    /// A retry may reuse an ID only for the identical room, author and content.
    /// INSERT conflict serialization guarantees concurrent requests create one outbox.
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
        let mut tx = self.pool.begin().await?;
        let result = Self::stage(&mut tx, id, room_id, content, author_kind, author_id).await?;
        tx.commit().await?;
        Ok(result)
    }
    async fn stage(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        id: Uuid,
        room_id: &str,
        content: &str,
        author_kind: &str,
        author_id: Option<&str>,
    ) -> Result<(Uuid, bool), sqlx::Error> {
        let event = ProductEvent::chat_message_persisted(id);
        let inserted = sqlx::query("INSERT INTO chat_messages(id, room_id, content, author_kind, author_id) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (id) DO NOTHING")
            .bind(id)
            .bind(room_id)
            .bind(content)
            .bind(author_kind)
            .bind(author_id)
            .execute(&mut **tx)
            .await?.rows_affected() == 1;
        if !inserted {
            let matches: bool = sqlx::query_scalar("SELECT room_id=$2 AND content=$3 AND author_kind=$4 AND author_id IS NOT DISTINCT FROM $5 FROM chat_messages WHERE id=$1")
                .bind(id).bind(room_id).bind(content).bind(author_kind).bind(author_id)
                .fetch_one(&mut **tx).await?;
            if !matches {
                return Err(sqlx::Error::Protocol("message id conflict".into()));
            }
            return Ok((id, false));
        }
        sqlx::query("INSERT INTO chat_outbox(event_id,message_id,schema_version,source_app,event_type,traceparent) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(event.event_id).bind(id).bind(i16::try_from(event.schema_version).unwrap())
            .bind(event.source_app).bind(event.event_type).bind(copilot_telemetry::outgoing_headers().get("traceparent")).execute(&mut **tx).await?;
        sqlx::query("SELECT pg_notify('copilot_chat_realtime',$1)")
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
        let mut tx = self.pool.begin().await?;
        let valid:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM chat_ai_room_locks WHERE room_id=$1 AND lease_token=$2 AND expires_at>now())").bind(room).bind(lease).fetch_one(&mut *tx).await?;
        if !valid {
            return Err(sqlx::Error::Protocol("persona lease expired".into()));
        }
        let id = Uuid::new_v4();
        let mut record = turn.clone();
        record["message_id"] = serde_json::json!(id);
        let updated=sqlx::query("UPDATE chat_persona_runs SET result=result||jsonb_build_array($2::jsonb),updated_at=now() WHERE id=$1 AND room_id=$3 AND status='running' AND jsonb_array_length(result)<max_turns AND reserved_tokens>=512*(jsonb_array_length(result)+1)").bind(run).bind(&record).bind(room).execute(&mut *tx).await?;
        if updated.rows_affected() != 1 {
            return Err(sqlx::Error::Protocol("persona run stopped".into()));
        }
        let updated=sqlx::query("UPDATE chat_personas SET memory=$1,last_spoke_at=now() WHERE id=$2 AND room_id=$3 AND revision=$4 AND enabled").bind(memory).bind(persona).bind(room).bind(revision).execute(&mut *tx).await?;
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
    #[sqlx::test(migrations = "./migrations")]
    async fn persona_memory_message_event_and_run_commit_only_with_current_authority(pool: PgPool) {
        let persona = Uuid::new_v4();
        let run = Uuid::new_v4();
        let lease = Uuid::new_v4();
        sqlx::query("INSERT INTO chat_rooms(id,name)VALUES('persona-test','Test')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chat_personas(id,room_id,name,instructions)VALUES($1,'persona-test','Coach','Teach')").bind(persona).execute(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO chat_ai_room_locks VALUES('persona-test',$1,now()+interval '5 minutes')",
        )
        .bind(lease)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO chat_persona_runs(id,room_id,parameters,status,max_turns,max_output_budget,reserved_tokens)VALUES($1,'persona-test','{}','running',1,512,512)").bind(run).execute(&pool).await.unwrap();
        let writer = ChatWriter::new(pool.clone());
        let turn = serde_json::json!({"answer":"Complete answer","memory":"bounded memory"});
        assert!(
            writer
                .persist_persona_turn("persona-test", persona, 99, run, lease, &turn)
                .await
                .is_err()
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM chat_messages")
                .fetch_one(&pool)
                .await
                .unwrap(),
            0
        );
        assert_eq!(
            sqlx::query_scalar::<_, serde_json::Value>("SELECT result FROM chat_persona_runs")
                .fetch_one(&pool)
                .await
                .unwrap(),
            serde_json::json!([])
        );
        assert!(
            writer
                .persist_persona_turn("persona-test", persona, 0, run, Uuid::new_v4(), &turn)
                .await
                .is_err()
        );
        let id = writer
            .persist_persona_turn("persona-test", persona, 0, run, lease, &turn)
            .await
            .unwrap();
        assert_eq!(
            sqlx::query_scalar::<_, String>("SELECT memory FROM chat_personas")
                .fetch_one(&pool)
                .await
                .unwrap(),
            "bounded memory"
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM chat_outbox WHERE event_id=$1")
                .bind(id)
                .fetch_one(&pool)
                .await
                .unwrap(),
            1
        );
        assert!(
            writer
                .persist_persona_turn("persona-test", persona, 0, run, lease, &turn)
                .await
                .is_err()
        );
        sqlx::query("UPDATE chat_persona_runs SET status='cancelled'")
            .execute(&pool)
            .await
            .unwrap();
        assert!(
            writer
                .persist_persona_turn("persona-test", persona, 0, run, lease, &turn)
                .await
                .is_err()
        );
    }
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
    #[sqlx::test(migrations = "./migrations")]
    async fn concurrent_message_replay_has_one_outbox_and_conflicts_are_rejected(pool: PgPool) {
        sqlx::query("INSERT INTO chat_rooms(id,name) VALUES ('replay','Test'),('other','Other')")
            .execute(&pool)
            .await
            .unwrap();
        let writer = ChatWriter::new(pool.clone());
        let id = Uuid::new_v4();
        let (a, b) = tokio::join!(
            writer.persist_with_id(id, "replay", "same", "user", Some("alice")),
            writer.persist_with_id(id, "replay", "same", "user", Some("alice"))
        );
        let a = a.unwrap();
        let b = b.unwrap();
        assert_eq!(a.0, b.0);
        assert_ne!(a.1, b.1);
        for (room, content, author) in [
            ("other", "same", "alice"),
            ("replay", "changed", "alice"),
            ("replay", "same", "bob"),
        ] {
            assert!(matches!(
                writer
                    .persist_with_id(id, room, content, "user", Some(author))
                    .await,
                Err(sqlx::Error::Protocol(_))
            ));
        }
        let counts: (i64, i64) = sqlx::query_as(
            "SELECT (SELECT count(*) FROM chat_messages),(SELECT count(*) FROM chat_outbox)",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(counts, (1, 1));
    }
}
