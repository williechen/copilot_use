use super::*;
#[sqlx::test(migrations = "./migrations")]
async fn persona_memory_message_event_and_run_commit_only_with_current_authority(pool: SqlitePool) {
    let persona = Uuid::new_v4();
    let run = Uuid::new_v4();
    let lease = Uuid::new_v4();
    sqlx::query("INSERT INTO chat_rooms(id,name)VALUES('persona-test','Test')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chat_personas(id,room_id,name,instructions)VALUES(?1,'persona-test','Coach','Teach')").bind(persona).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO chat_ai_room_locks VALUES('persona-test',?1,unixepoch('subsec')+300)")
        .bind(lease)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chat_persona_runs(id,room_id,parameters,status,max_turns,max_output_budget,reserved_tokens)VALUES(?1,'persona-test','{}','running',1,512,512)").bind(run).execute(&pool).await.unwrap();
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
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM chat_outbox WHERE event_id=?1")
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
async fn message_insert_has_exactly_one_outbox_event(pool: SqlitePool) {
    sqlx::query("INSERT INTO chat_rooms(id,name) VALUES ('room-1','Test')")
        .execute(&pool)
        .await
        .unwrap();
    let id = ChatWriter::new(pool.clone())
        .persist("room-1", "private")
        .await
        .unwrap();
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM chat_outbox WHERE event_id=?1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.0, 1);
}
#[sqlx::test(migrations = "./migrations")]
async fn missing_room_rolls_back_both_message_and_event(pool: SqlitePool) {
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
async fn stale_lease_cannot_ack(pool: SqlitePool) {
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
    sqlx::query("UPDATE chat_outbox SET lease_until=unixepoch('subsec')-1 WHERE event_id=?1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    let fresh = store.claim_one().await.unwrap().unwrap();
    assert!(!store.ack(&stale).await.unwrap());
    assert!(store.ack(&fresh).await.unwrap());
}
#[sqlx::test(migrations = "./migrations")]
async fn outbox_insert_failure_rolls_back_inserted_message(pool: SqlitePool) {
    sqlx::query("INSERT INTO chat_rooms(id,name) VALUES ('rollback','Test')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE TRIGGER reject_outbox BEFORE INSERT ON chat_outbox BEGIN SELECT RAISE(ABORT,'reject'); END;")
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
async fn expired_lease_can_be_reclaimed_and_old_token_cannot_mutate(pool: SqlitePool) {
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
    sqlx::query("UPDATE chat_outbox SET lease_until=unixepoch('subsec')-1")
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
async fn permanent_delivery_failure_is_blocked(pool: SqlitePool) {
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
async fn concurrent_message_replay_has_one_outbox_and_conflicts_are_rejected(pool: SqlitePool) {
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
