//! Portable message commit use case: only a durable Repository result may reach the caller.
use std::{future::Future, pin::Pin};
use uuid::Uuid;
pub struct MessageInput<'a> {
    pub id: Uuid,
    pub room: &'a str,
    pub content: &'a str,
    pub author_kind: &'a str,
    pub author_id: Option<&'a str>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Committed {
    pub id: Uuid,
    pub inserted: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageError {
    InvalidInput,
    Conflict,
    Unavailable,
}
pub type CommitFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Committed, MessageError>> + Send + 'a>>;
pub trait MessageRepository: Send + Sync {
    fn commit<'a>(&'a self, input: MessageInput<'a>) -> CommitFuture<'a>;
}
pub async fn persist(
    repository: &impl MessageRepository,
    input: MessageInput<'_>,
) -> Result<Committed, MessageError> {
    let valid_author = match input.author_kind {
        "anonymous_user" | "system" => input.author_id.is_none(),
        "user" | "ai" => input
            .author_id
            .is_some_and(|id| !id.trim().is_empty() && id.len() <= 255 && !id.contains('\0')),
        _ => false,
    };
    if input.id.is_nil()
        || input.room.is_empty()
        || input.room.len() > 40
        || input.room.contains('\0')
        || input.content.trim().is_empty()
        || input.content.len() > 65536
        || input.content.contains('\0')
        || !valid_author
    {
        return Err(MessageError::InvalidInput);
    }
    repository.commit(input).await
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Failed;
    impl MessageRepository for Failed {
        fn commit<'a>(&'a self, _: MessageInput<'a>) -> CommitFuture<'a> {
            Box::pin(async { Err(MessageError::Unavailable) })
        }
    }
    #[tokio::test]
    async fn failed_commit_never_returns_a_publishable_message() {
        let input = MessageInput {
            id: Uuid::new_v4(),
            room: "fixture",
            content: "complete text",
            author_kind: "anonymous_user",
            author_id: None,
        };
        assert_eq!(
            persist(&Failed, input).await,
            Err(MessageError::Unavailable)
        );
    }
}
