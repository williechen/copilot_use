//! Analytics 接收與查詢用例；事件驗證後才進入已接線的 Repository。
//! 參考 valid_event／ingest 的驗證邊界在此收斂，認證、PostgreSQL 與 gRPC 留在 Adapter。
use event_contract::ProductEvent;
use serde::Serialize;
use std::{future::Future, pin::Pin};
use uuid::Uuid;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryError {
    InvalidEvent,
    IdentityConflict,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EventSummary {
    pub source_app: String,
    pub event_type: String,
    pub total: i64,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EventRecord {
    pub event_id: Uuid,
    pub source_app: String,
    pub event_type: String,
    pub received_unix_ms: i64,
}
pub type RepositoryFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, RepositoryError>> + Send + 'a>>;

/// 交易成功才回傳結果；持久冪等不由記憶體去重器取代。
pub trait EventRepository: Send + Sync {
    fn insert_once<'a>(&'a self, event: &'a ProductEvent) -> RepositoryFuture<'a, bool>;
    fn recent<'a>(
        &'a self,
        source: Option<&'a str>,
        kind: Option<&'a str>,
    ) -> RepositoryFuture<'a, Vec<EventRecord>>;
    fn total(&self) -> RepositoryFuture<'_, i64>;
    fn summary(&self) -> RepositoryFuture<'_, Vec<EventSummary>>;
    fn lookup<'a>(&'a self, source: &'a str, id: Uuid) -> RepositoryFuture<'a, Option<String>>;
}

/// 純用例協調者，不持有 PgPool、HTTP 或 gRPC Driver。
pub struct Analytics<R: EventRepository> {
    repository: R,
}
impl<R: EventRepository> Analytics<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub async fn receive(&self, event: &ProductEvent) -> Result<bool, RepositoryError> {
        if !event.is_valid() {
            return Err(RepositoryError::InvalidEvent);
        }
        self.repository.insert_once(event).await
    }
    pub async fn recent(
        &self,
        source: Option<&str>,
        kind: Option<&str>,
    ) -> Result<Vec<EventRecord>, RepositoryError> {
        self.repository.recent(source, kind).await
    }
    pub async fn total(&self) -> Result<i64, RepositoryError> {
        self.repository.total().await
    }
    pub async fn summary(&self) -> Result<Vec<EventSummary>, RepositoryError> {
        self.repository.summary().await
    }
    pub async fn lookup(&self, source: &str, id: Uuid) -> Result<Option<String>, RepositoryError> {
        self.repository.lookup(source, id).await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    struct Unavailable {
        called: Arc<AtomicBool>,
    }
    impl EventRepository for Unavailable {
        fn insert_once<'a>(&'a self, _: &'a ProductEvent) -> RepositoryFuture<'a, bool> {
            Box::pin(async move {
                self.called.store(true, Ordering::SeqCst);
                Err(RepositoryError::Unavailable)
            })
        }
        fn recent<'a>(
            &'a self,
            _: Option<&'a str>,
            _: Option<&'a str>,
        ) -> RepositoryFuture<'a, Vec<EventRecord>> {
            Box::pin(async { Err(RepositoryError::Unavailable) })
        }
        fn total(&self) -> RepositoryFuture<'_, i64> {
            Box::pin(async { Err(RepositoryError::Unavailable) })
        }
        fn summary(&self) -> RepositoryFuture<'_, Vec<EventSummary>> {
            Box::pin(async { Err(RepositoryError::Unavailable) })
        }
        fn lookup<'a>(&'a self, _: &'a str, _: Uuid) -> RepositoryFuture<'a, Option<String>> {
            Box::pin(async { Err(RepositoryError::Unavailable) })
        }
    }
    #[tokio::test]
    async fn malformed_event_never_reaches_repository_and_storage_failure_is_not_success() {
        let called = Arc::new(AtomicBool::new(false));
        let app = Analytics::new(Unavailable {
            called: called.clone(),
        });
        let mut event = ProductEvent::chat_message_persisted(Uuid::nil());
        assert_eq!(
            app.receive(&event).await,
            Err(RepositoryError::InvalidEvent)
        );
        assert!(!called.load(Ordering::SeqCst));
        event.event_id = Uuid::new_v4();
        assert_eq!(app.receive(&event).await, Err(RepositoryError::Unavailable));
        assert!(called.load(Ordering::SeqCst));
    }
}
