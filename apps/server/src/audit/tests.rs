use super::*;
use serde_json::{Value, json};
use std::sync::Arc;
struct Fixture {
    url: String,
    pool: sqlx::SqlitePool,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.task.abort();
    }
}
async fn fixture() -> Fixture {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("../../crates/persistence/migrations")
        .run(&pool)
        .await
        .unwrap();
    let state = StateData {
        pool: pool.clone(),
        api_token: Arc::from("fixture-admin-not-an-audit-reader"),
        audit_token: Arc::from("fixture-audit-reader-only"),
        events: tokio::sync::broadcast::channel(8).0,
        ai_slots: Arc::new(tokio::sync::Semaphore::new(1)),
        auth_slots: Arc::new(tokio::sync::Semaphore::new(2)),
        ws_slots: Arc::new(tokio::sync::Semaphore::new(2)),
        tool_slots: Arc::new(tokio::sync::Semaphore::new(1)),
        mcp_endpoint: Arc::from("http://127.0.0.1/unused"),
    };
    let app = axum::Router::new()
        .route("/register", axum::routing::post(crate::auth::register))
        .route("/login", axum::routing::post(crate::auth::login))
        .route("/logout", axum::routing::post(crate::auth::logout))
        .route("/room", axum::routing::post(crate::create_room))
        .route("/audit", axum::routing::post(query))
        .route("/rooms", axum::routing::get(crate::list_rooms))
        .route("/public/rooms", axum::routing::get(crate::public_rooms))
        .route(
            "/messages",
            axum::routing::get(crate::list_messages).post(crate::send_message),
        )
        .route("/join/{id}", axum::routing::post(crate::management::join))
        .route(
            "/members/{id}",
            axum::routing::get(crate::management::members),
        )
        .route("/mine", axum::routing::get(crate::management::rooms))
        .route(
            "/personas/{room}",
            axum::routing::get(crate::personas::list),
        )
        .with_state(state);
    let socket = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", socket.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(socket, app).await.unwrap();
    });
    Fixture { url, pool, task }
}
async fn post(f: &Fixture, path: &str, body: Value, cookie: Option<&str>) -> reqwest::Response {
    let mut r = reqwest::Client::new()
        .post(format!("{}{path}", f.url))
        .header("X-Copilot-CSRF", "1")
        .json(&body);
    if let Some(cookie) = cookie {
        r = r.header("cookie", cookie);
    }
    r.send().await.unwrap()
}
async fn audit(f: &Fixture, body: Value) -> Vec<AuditRecord> {
    reqwest::Client::new()
        .post(format!("{}/audit", f.url))
        .bearer_auth("fixture-audit-reader-only")
        .json(&body)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap()
}
#[tokio::test]
async fn identities_filters_authorization_and_transactional_rollback() {
    let f = fixture().await;
    let credentials = json!({"username":"audit_alice","password":"synthetic-fixture-password"});
    assert_eq!(
        post(&f, "/register", credentials.clone(), None)
            .await
            .status(),
        201
    );
    assert_eq!(
        post(
            &f,
            "/login",
            json!({"username":"audit_alice","password":"synthetic-wrong-password"}),
            None
        )
        .await
        .status(),
        401
    );
    assert_eq!(
        post(
            &f,
            "/login",
            json!({"username":"audit_unknown","password":"synthetic-wrong-password"}),
            None
        )
        .await
        .status(),
        401
    );
    let logged = post(&f, "/login", credentials.clone(), None).await;
    assert_eq!(logged.status(), 200);
    let cookie = logged.headers()["set-cookie"]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_owned();
    for token in [None, Some("fixture-admin-not-an-audit-reader")] {
        let mut r = reqwest::Client::new()
            .post(format!("{}/audit", f.url))
            .header("cookie", &cookie)
            .json(&json!({}));
        if let Some(token) = token {
            r = r.bearer_auth(token);
        }
        assert_eq!(r.send().await.unwrap().status(), 401);
    }
    let body = json!({"id":"audit-room","name":"稽核測試房間"});
    assert_eq!(
        post(&f, "/room", body.clone(), Some(&cookie))
            .await
            .status(),
        201
    );
    assert_eq!(post(&f, "/room", body, Some(&cookie)).await.status(), 409);
    let rooms = audit(
        &f,
        json!({"event_type":"room.created","username":"audit_alice","succeeded":true}),
    )
    .await;
    assert_eq!(rooms.len(), 1);
    assert_eq!(rooms[0].room_name.as_deref(), Some("稽核測試房間"));
    assert!(rooms[0].user_id.is_some());
    let event: String = sqlx::query_scalar("SELECT event_type FROM chat_outbox WHERE event_id=?1")
        .bind(rooms[0].event_id)
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(event, "room.created");
    assert_eq!(audit(&f, json!({"succeeded":false})).await.len(), 2);
    assert_eq!(audit(&f, json!({"username":"audit_alice"})).await.len(), 3);
    let boundary = rooms[0].occurred_unix_ms;
    assert!(
        audit(
            &f,
            json!({"from_unix_ms":boundary,"to_unix_ms":boundary+1,"event_type":"room.created"})
        )
        .await
        .len()
            == 1
    );
    assert!(
        audit(&f, json!({"username":"not_this_user"}))
            .await
            .is_empty()
    );
    assert_eq!(
        post(&f, "/logout", json!({}), Some(&cookie)).await.status(),
        204
    );
    assert_eq!(
        audit(&f, json!({"event_type":"logout.succeeded"}))
            .await
            .len(),
        1
    );
    let rows = audit(&f, json!({})).await;
    let text = serde_json::to_string(&rows).unwrap();
    assert!(!text.contains("password") && !text.contains(&cookie) && !text.contains("token"));
    // Audit storage failure must roll back both the operation and its public event.
    sqlx::raw_sql("CREATE TRIGGER reject_audit BEFORE INSERT ON chat_activity_audit BEGIN SELECT RAISE(ABORT,'fixture audit unavailable'); END;").execute(&f.pool).await.unwrap();
    assert_eq!(post(&f, "/login", credentials, None).await.status(), 503);
    let sessions: i64 = sqlx::query_scalar("SELECT count(*) FROM chat_sessions")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(sessions, 0);
    let created = reqwest::Client::new()
        .post(format!("{}/room", f.url))
        .bearer_auth("fixture-admin-not-an-audit-reader")
        .json(&json!({"id":"must-rollback","name":"必須回滾"}))
        .send()
        .await
        .unwrap();
    assert_eq!(created.status(), 503);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM chat_rooms WHERE id='must-rollback'")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    assert_eq!(audit(&f, json!({})).await.len(), rows.len());
}
#[tokio::test]
async fn additive_migration_preserves_all_outbox_columns_and_delivery_states() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(include_str!(
        "../../../../crates/persistence/migrations/202610120001_portable_chat.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    for status in ["pending", "leased", "delivered", "blocked"] {
        sqlx::query("INSERT INTO chat_outbox(event_id,schema_version,source_app,event_type,status,attempts,lease_token,lease_until,delivered_at,last_error,traceparent)VALUES(?1,1,'web_chat','login.succeeded',?2,3,?3,?4,?5,'preserve-fixture','00-11111111111111111111111111111111-2222222222222222-01')").bind(Uuid::new_v4()).bind(status).bind((status=="leased").then(Uuid::new_v4)).bind((status=="leased").then_some(99.5)).bind((status=="delivered").then_some(88.5)).execute(&pool).await.unwrap();
    }
    let query = "SELECT json_array(hex(event_id),hex(message_id),schema_version,source_app,event_type,status,attempts,next_attempt_at,hex(lease_token),lease_until,delivered_at,last_error,created_at,traceparent) FROM chat_outbox ORDER BY event_id";
    let before: Vec<String> = sqlx::query_scalar(query).fetch_all(&pool).await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    sqlx::raw_sql(include_str!(
        "../../../../crates/persistence/migrations/202610170001_chat_activity_audit.sql"
    ))
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    let after: Vec<String> = sqlx::query_scalar(query).fetch_all(&pool).await.unwrap();
    assert_eq!(before, after);
    assert_eq!(
        sqlx::query_scalar::<_, String>("PRAGMA integrity_check")
            .fetch_one(&pool)
            .await
            .unwrap(),
        "ok"
    );
    assert!(
        sqlx::query("PRAGMA foreign_key_check")
            .fetch_all(&pool)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn public_room_and_private_room_follow_the_same_basic_user_journey() {
    let f = fixture().await;
    let a = json!({"username":"journey_alice","password":"synthetic-fixture-password"});
    let b = json!({"username":"journey_bob","password":"synthetic-fixture-password"});
    assert_eq!(post(&f, "/register", a.clone(), None).await.status(), 201);
    assert_eq!(post(&f, "/register", b.clone(), None).await.status(), 201);
    let cookie = |r: &reqwest::Response| {
        r.headers()["set-cookie"]
            .to_str()
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .to_owned()
    };
    let alice = cookie(&post(&f, "/login", a, None).await);
    let bob = cookie(&post(&f, "/login", b, None).await);
    assert_eq!(
        post(
            &f,
            "/room",
            json!({"id":"open-room","name":"公開房間"}),
            Some(&alice)
        )
        .await
        .status(),
        201
    );
    assert_eq!(
        post(
            &f,
            "/room",
            json!({"id":"private-room","name":"私聊房間","is_private":true}),
            Some(&alice)
        )
        .await
        .status(),
        201
    );
    let get = |path: String, cookie: String| {
        reqwest::Client::new()
            .get(format!("{}{path}", f.url))
            .header("cookie", cookie)
            .send()
    };
    let rooms: Value = get("/rooms".into(), bob.clone())
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(rooms.as_array().unwrap().len(), 1);
    assert_eq!(rooms[0]["id"], "open-room");
    assert_eq!(
        post(&f, "/join/open-room", json!({}), Some(&bob))
            .await
            .status(),
        204
    );
    assert_eq!(
        post(&f, "/join/private-room", json!({}), Some(&bob))
            .await
            .status(),
        403
    );
    assert_eq!(
        get("/members/open-room".into(), bob.clone())
            .await
            .unwrap()
            .status(),
        403
    );
    let mine: Value = get("/mine".into(), bob.clone())
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(mine.as_array().unwrap().is_empty());
    assert_eq!(
        get("/personas/open-room".into(), alice.clone())
            .await
            .unwrap()
            .status(),
        403
    );
    assert_eq!(
        reqwest::Client::new()
            .get(format!("{}/personas/open-room", f.url))
            .bearer_auth("fixture-admin-not-an-audit-reader")
            .send()
            .await
            .unwrap()
            .status(),
        200
    );
    let id = Uuid::new_v4();
    assert_eq!(
        post(
            &f,
            "/messages",
            json!({"id":id,"room_id":"open-room","content":"公開房間基本流程"}),
            Some(&bob)
        )
        .await
        .status(),
        201
    );
    let messages: Value = get("/messages?room_id=open-room".into(), alice)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(messages[0]["id"], id.to_string());
    assert_eq!(messages[0]["content"], "公開房間基本流程");
    assert_eq!(
        get("/messages?room_id=private-room".into(), bob)
            .await
            .unwrap()
            .status(),
        403
    );
}

#[tokio::test]
async fn anonymous_directory_exposes_only_public_names_without_granting_entry() {
    let f = fixture().await;
    let credentials = json!({"username":"directory_owner","password":"synthetic-fixture-password"});
    let registered = post(&f, "/register", credentials.clone(), None).await;
    assert_eq!(registered.status(), 201);
    assert!(
        registered.headers().get("set-cookie").is_none(),
        "registration must not sign in"
    );
    let login = post(&f, "/login", credentials, None).await;
    let cookie = login.headers()["set-cookie"]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_owned();
    for (id, private) in [("visible", false), ("hidden", true)] {
        assert_eq!(
            post(
                &f,
                "/room",
                json!({"id":id,"name":id,"is_private":private}),
                Some(&cookie)
            )
            .await
            .status(),
            201
        );
    }
    let response = reqwest::Client::new()
        .get(format!("{}/public/rooms", f.url))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let rows: Value = response.json().await.unwrap();
    assert_eq!(rows, json!([{"id":"visible","name":"visible"}]));
    assert_eq!(
        post(&f, "/join/visible", json!({}), None).await.status(),
        401
    );
    let stranger = json!({"username":"directory_guest","password":"synthetic-fixture-password"});
    assert_eq!(
        post(&f, "/register", stranger.clone(), None).await.status(),
        201
    );
    let login = post(&f, "/login", stranger, None).await;
    let guest = login.headers()["set-cookie"]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_owned();
    assert_eq!(
        post(&f, "/join/visible", json!({}), Some(&guest))
            .await
            .status(),
        204
    );
    assert_eq!(
        post(&f, "/join/hidden", json!({}), Some(&guest))
            .await
            .status(),
        403
    );
}
