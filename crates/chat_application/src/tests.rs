use super::*;
use uuid::Uuid;
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
