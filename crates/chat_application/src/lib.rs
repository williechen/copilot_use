//! 訊息持久化用例：先驗證，再取得 Repository 的 durable 收據。
//! 對應參考 MessageRepository／persist_then_publish 的先存後通知職責；
//! 成品通知由同交易 realtime／Outbox 保證，不以記憶體 callback 取代。
pub use chat_domain::{Committed, MessageError, MessageInput};
use std::{future::Future, pin::Pin};

/// Repository 的非同步結果，保留精確 ID 與是否為首次寫入。
pub type CommitFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Committed, MessageError>> + Send + 'a>>;

/// 已接線的寫入 Port；SQL、交易與事件投遞留在 persistence Adapter。
pub trait MessageRepository: Send + Sync {
    fn commit<'a>(&'a self, input: MessageInput<'a>) -> CommitFuture<'a>;
}

/// 只有有效輸入能進入 Port，儲存失敗不產生可發布的成功收據。
pub async fn persist(
    repository: &impl MessageRepository,
    input: MessageInput<'_>,
) -> Result<Committed, MessageError> {
    chat_domain::validate(&input)?;
    repository.commit(input).await
}

#[cfg(test)]
mod tests;
