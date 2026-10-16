use super::*;
use crate::{AnalyticsState, build_app};
use event_contract::ProductEvent;
use futures_util::StreamExt;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;
const CHAT: &str = "grpc-chat-test-credential";
const PET: &str = "grpc-pet-test-credential";
const READ: &str = "grpc-read-test-credential";
struct Server {
    url: String,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}
async fn server(pool: PgPool) -> Server {
    let state = AnalyticsState {
        pool,
        api_token: Arc::from(READ),
        chat_token: Arc::from(CHAT),
        pet_token: Arc::from(PET),
        admin_token: Arc::from("grpc-admin-test-credential"),
        audit_source: None,
        ai_slots: Arc::new(tokio::sync::Semaphore::new(2)),
    };
    let socket = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", socket.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(socket, build_app(state)).await.unwrap();
    });
    Server { url, task }
}
async fn ingest(
    url: &str,
    token: &str,
    events: Vec<proto::ProductEvent>,
) -> Result<proto::IngestEventsResponse, Status> {
    let mut client = proto::event_ingestion_client::EventIngestionClient::connect(url.to_owned())
        .await
        .unwrap();
    let mut request = Request::new(proto::IngestEventsRequest { events });
    request
        .metadata_mut()
        .insert("authorization", format!("Bearer {token}").parse().unwrap());
    client
        .ingest_events(request)
        .await
        .map(Response::into_inner)
}
async fn revision(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT revision FROM analytics_revision")
        .fetch_one(pool)
        .await
        .unwrap()
}
#[sqlx::test(migrations = "./migrations")]
async fn real_grpc_batch_auth_conflict_concurrency_and_http_query(pool: PgPool) {
    let server = server(pool.clone()).await;
    let event = ProductEvent::chat_message_persisted(Uuid::new_v4());
    for key in [READ, PET, "wrong"] {
        assert_eq!(
            ingest(&server.url, key, vec![(&event).into()])
                .await
                .unwrap_err()
                .code(),
            tonic::Code::Unauthenticated
        );
    }
    let pet = ProductEvent::pet_cleanup_succeeded(Uuid::new_v4());
    assert_eq!(
        ingest(&server.url, CHAT, vec![(&event).into(), (&pet).into()])
            .await
            .unwrap_err()
            .code(),
        tonic::Code::Unauthenticated
    );
    assert_eq!(EventStore::new(pool.clone()).total().await.unwrap(), 0);
    for events in [vec![], vec![(&event).into(); MAX_BATCH + 1]] {
        assert_eq!(
            ingest(&server.url, CHAT, events).await.unwrap_err().code(),
            tonic::Code::InvalidArgument
        );
    }
    let (a, b) = tokio::join!(
        ingest(&server.url, CHAT, vec![(&event).into()]),
        ingest(&server.url, CHAT, vec![(&event).into()])
    );
    let a = a.unwrap().results[0].outcome;
    let b = b.unwrap().results[0].outcome;
    assert_ne!(a, b);
    assert_eq!(revision(&pool).await, 1);
    let mut conflict: proto::ProductEvent = (&event).into();
    conflict.event_type = "login.succeeded".into();
    let mut invalid: proto::ProductEvent = (&event).into();
    invalid.schema_version = 65537;
    let mut nil: proto::ProductEvent = (&event).into();
    nil.event_id = Uuid::nil().to_string();
    let response = ingest(&server.url, CHAT, vec![conflict, invalid, nil])
        .await
        .unwrap();
    assert_eq!(
        response
            .results
            .iter()
            .map(|r| r.outcome)
            .collect::<Vec<_>>(),
        vec![
            proto::Outcome::IdentityConflict as i32,
            proto::Outcome::InvalidEvent as i32,
            proto::Outcome::InvalidEvent as i32
        ]
    );
    let batch = (0..MAX_BATCH)
        .map(|_| proto::ProductEvent::from(&ProductEvent::chat_message_persisted(Uuid::new_v4())))
        .collect();
    assert_eq!(
        ingest(&server.url, CHAT, batch)
            .await
            .unwrap()
            .results
            .len(),
        MAX_BATCH
    );
    let mut large: proto::ProductEvent = (&event).into();
    large.event_type = "x".repeat(MAX_MESSAGE_BYTES);
    assert!(ingest(&server.url, CHAT, vec![large]).await.is_err());
    let response = reqwest::Client::new()
        .get(format!("{}/api/v1/events/count", server.url))
        .bearer_auth(READ)
        .send()
        .await
        .unwrap();
    assert_eq!(
        response.json::<serde_json::Value>().await.unwrap()["total"],
        65
    );
    // Restart HTTP/gRPC service and redeliver the same ID through the production client.
    drop(server);
    let restarted = self::server(pool.clone()).await;
    analytics_transport::deliver(&restarted.url, CHAT, &event, None)
        .await
        .unwrap();
    assert_eq!(EventStore::new(pool).total().await.unwrap(), 65);
}

#[sqlx::test(migrations = "./migrations")]
async fn real_sqlite_outboxes_retry_and_ack_only_committed_grpc_events(pool: PgPool) {
    let server = server(pool.clone()).await;
    let dir = tempfile::tempdir().unwrap();
    let url = format!(
        "sqlite://{}",
        dir.path().join("private/chat.sqlite3").display()
    );
    let chat = persistence::database::open(&url).await.unwrap();
    sqlx::query("INSERT INTO chat_rooms(id,name)VALUES('grpc','gRPC')")
        .execute(&chat)
        .await
        .unwrap();
    let id = persistence::ChatWriter::new(chat.clone())
        .persist("grpc", "private content stays local")
        .await
        .unwrap();
    let store = persistence::OutboxStore::new(chat.clone());
    let client = reqwest::Client::new();
    persistence::dispatch_once(&store, &client, "http://127.0.0.1:9", CHAT)
        .await
        .unwrap();
    let status: String = sqlx::query_scalar("SELECT status FROM chat_outbox WHERE event_id=?")
        .bind(id)
        .fetch_one(&chat)
        .await
        .unwrap();
    assert_eq!(status, "pending");
    sqlx::query("UPDATE chat_outbox SET next_attempt_at=0")
        .execute(&chat)
        .await
        .unwrap();
    // Simulate commit succeeded but response lost: preinsert, then worker receives ALREADY_EXISTS.
    EventStore::new(pool.clone())
        .insert_once(&ProductEvent::chat_message_persisted(id))
        .await
        .unwrap();
    persistence::dispatch_once(&store, &client, &server.url, CHAT)
        .await
        .unwrap();
    let status: String = sqlx::query_scalar("SELECT status FROM chat_outbox WHERE event_id=?")
        .bind(id)
        .fetch_one(&chat)
        .await
        .unwrap();
    assert_eq!(status, "delivered");
    let pet = day9_pet::state::PetStore::open(&dir.path().join("pet.sqlite3"))
        .await
        .unwrap();
    let receipt = day9_pet::CleanupReceipt {
        operation_id: Uuid::new_v4(),
        quarantined_file: "private-test.item".into(),
        size_bytes: 1,
    };
    pet.record_cleanup(&receipt, &receipt.event())
        .await
        .unwrap();
    pet.dispatch_once("http://127.0.0.1:9", PET).await.unwrap();
    assert_eq!(pet.outbox_status().await.unwrap()["delivered"], 0);
    // Backoff is bounded; actual worker retries after the first delay.
    tokio::time::sleep(Duration::from_secs(3)).await;
    pet.dispatch_once(&server.url, PET).await.unwrap();
    assert_eq!(pet.outbox_status().await.unwrap()["delivered"], 1);
    assert_eq!(EventStore::new(pool).total().await.unwrap(), 2);
    chat.close().await;
}

#[sqlx::test(migrations = "./migrations")]
async fn sse_observes_commit_not_rollback_dedup_and_reconnect_snapshot(pool: PgPool) {
    let server = server(pool.clone()).await;
    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/events/stream", server.url);
    assert_eq!(client.get(&url).send().await.unwrap().status(), 401);
    assert_eq!(
        client
            .get(&url)
            .bearer_auth(CHAT)
            .send()
            .await
            .unwrap()
            .status(),
        401
    );
    let response = client.get(&url).bearer_auth(READ).send().await.unwrap();
    assert!(
        response.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("text/event-stream")
    );
    let mut stream = response.bytes_stream();
    let first = stream.next().await.unwrap().unwrap();
    assert!(String::from_utf8_lossy(&first).contains("event: invalidate"));
    let event = ProductEvent::chat_message_persisted(Uuid::new_v4());
    analytics_transport::deliver(&server.url, CHAT, &event, None)
        .await
        .unwrap();
    let next = tokio::time::timeout(Duration::from_secs(3), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(String::from_utf8_lossy(&next).contains("data: 1"));
    analytics_transport::deliver(&server.url, CHAT, &event, None)
        .await
        .unwrap();
    assert_eq!(revision(&pool).await, 1);
    assert!(
        tokio::time::timeout(Duration::from_millis(1200), stream.next())
            .await
            .is_err()
    );
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("UPDATE product_events SET archived=true")
        .execute(&mut *tx)
        .await
        .unwrap();
    assert_eq!(revision(&pool).await, 1);
    tx.rollback().await.unwrap();
    assert_eq!(revision(&pool).await, 1);
    sqlx::query("UPDATE product_events SET archived=true")
        .execute(&pool)
        .await
        .unwrap();
    let next = tokio::time::timeout(Duration::from_secs(3), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(String::from_utf8_lossy(&next).contains("data: 2"));
    drop(stream);
    let mut reconnected = client
        .get(&url)
        .bearer_auth(READ)
        .send()
        .await
        .unwrap()
        .bytes_stream();
    let snapshot = reconnected.next().await.unwrap().unwrap();
    assert!(String::from_utf8_lossy(&snapshot).contains("data: 2"));
}

#[tokio::test]
async fn database_failure_is_unavailable_without_success_ack_or_sse() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(100))
        .connect_lazy("postgres://unused@127.0.0.1:9/unused")
        .unwrap();
    let server = server(pool).await;
    let event = ProductEvent::chat_message_persisted(Uuid::new_v4());
    assert_eq!(
        analytics_transport::deliver(&server.url, CHAT, &event, None)
            .await
            .unwrap_err()
            .code(),
        tonic::Code::Unavailable
    );
    assert_eq!(
        reqwest::Client::new()
            .get(format!("{}/api/v1/events/stream", server.url))
            .bearer_auth(READ)
            .send()
            .await
            .unwrap()
            .status(),
        503
    );
}
