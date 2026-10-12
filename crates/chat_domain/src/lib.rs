//! 訊息資料與不變量；不依賴資料庫、OS 或 UI。
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
pub fn validate(input: &MessageInput<'_>) -> Result<(), MessageError> {
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
    Ok(())
}
