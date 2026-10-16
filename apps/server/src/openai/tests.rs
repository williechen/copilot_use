//! 授權／交易回歸使用本地完成文字 fixture，不呼叫 Provider 或讀取密鑰。
use super::*;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use uuid::Uuid;

async fn fixture() -> (StateData, HeaderMap, Uuid) {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("../../crates/persistence/migrations")
        .run(&pool)
        .await
        .unwrap();
    let user = Uuid::new_v4();
    sqlx::query("INSERT INTO chat_users VALUES(?1,'fixture','unused')")
        .bind(user)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chat_rooms(id,name) VALUES('private','fixture')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chat_room_members VALUES('private',?1)")
        .bind(user)
        .execute(&pool)
        .await
        .unwrap();
    let token = "ab".repeat(32);
    sqlx::query("INSERT INTO chat_sessions VALUES(?1,?2,unixepoch('subsec')+600)")
        .bind(Sha256::digest(token.as_bytes()).to_vec())
        .bind(user)
        .execute(&pool)
        .await
        .unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        "cookie",
        format!("copilot_session={token}").parse().unwrap(),
    );
    headers.insert("x-copilot-csrf", "1".parse().unwrap());
    let state = StateData {
        pool,
        api_token: Arc::from("fixture-admin-not-a-real-secret"),
        events: tokio::sync::broadcast::channel(8).0,
        ai_slots: Arc::new(tokio::sync::Semaphore::new(1)),
        auth_slots: Arc::new(tokio::sync::Semaphore::new(1)),
        ws_slots: Arc::new(tokio::sync::Semaphore::new(1)),
        tool_slots: Arc::new(tokio::sync::Semaphore::new(1)),
        mcp_endpoint: Arc::from("http://127.0.0.1/unused"),
    };
    (state, headers, user)
}

#[tokio::test]
async fn completion_rejects_revoked_session_membership_expiry_and_csrf_without_side_effects() {
    for change in [
        "DELETE FROM chat_sessions",
        "DELETE FROM chat_room_members",
        "UPDATE chat_sessions SET expires_at=0",
        "csrf",
    ] {
        let (state, mut headers, _) = fixture().await;
        auth::principal(&state, &headers, true).await.unwrap();
        if change == "csrf" {
            headers.remove("x-copilot-csrf");
        } else {
            sqlx::query(change).execute(&state.pool).await.unwrap();
        }
        assert!(
            persist_completed(
                &state,
                &headers,
                "private",
                "完成文字 fixture",
                "fixture-model"
            )
            .await
            .is_err(),
            "{change}"
        );
        for query in [
            "SELECT count(*) FROM chat_messages",
            "SELECT count(*) FROM chat_outbox",
            "SELECT count(*) FROM chat_realtime",
        ] {
            let count: i64 = sqlx::query_scalar(query)
                .fetch_one(&state.pool)
                .await
                .unwrap();
            assert_eq!(count, 0, "{change}: {query}");
        }
    }
}

#[tokio::test]
async fn authorized_completion_commits_message_event_and_realtime_together() {
    let (state, headers, _) = fixture().await;
    let id = persist_completed(
        &state,
        &headers,
        "private",
        "完成文字 fixture",
        "fixture-model",
    )
    .await
    .unwrap();
    let saved: String = sqlx::query_scalar("SELECT content FROM chat_messages WHERE id=?1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(saved, "完成文字 fixture");
    for query in [
        "SELECT count(*) FROM chat_outbox",
        "SELECT count(*) FROM chat_realtime",
    ] {
        assert_eq!(
            sqlx::query_scalar::<_, i64>(query)
                .fetch_one(&state.pool)
                .await
                .unwrap(),
            1
        );
    }
}
