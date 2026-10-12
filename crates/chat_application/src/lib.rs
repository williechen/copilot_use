//! Portable message commit use case: only a durable Repository result may reach the caller.
pub use chat_domain::{Committed, MessageError, MessageInput};
use std::{future::Future, pin::Pin};
pub type CommitFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Committed, MessageError>> + Send + 'a>>;
pub trait MessageRepository: Send + Sync {
    fn commit<'a>(&'a self, input: MessageInput<'a>) -> CommitFuture<'a>;
}
pub async fn persist(
    repository: &impl MessageRepository,
    input: MessageInput<'_>,
) -> Result<Committed, MessageError> {
    chat_domain::validate(&input)?;
    repository.commit(input).await
}
#[cfg(test)]
mod tests;
