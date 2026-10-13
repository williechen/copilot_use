//! 訊息資料與不變量；不依賴資料庫、OS 或 UI。
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageError {
    InvalidInput,
    Conflict,
    Unavailable,
}

/// 作者、房間及本文的寫入輸入；由入口 Adapter 映射既有協定。
pub struct MessageInput<'a> {
    pub id: Uuid,
    pub room: &'a str,
    pub content: &'a str,
    pub author_kind: &'a str,
    pub author_id: Option<&'a str>,
}

/// 持久化成功後的收據；相同訊息重送保留 inserted=false 的語意。
#[derive(Debug, PartialEq, Eq)]
pub struct Committed {
    pub id: Uuid,
    pub inserted: bool,
}

/// 對應參考 MessageAuthor／ChatMessage 的輸入不變量；不讀 DB 或系統時鐘。
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
