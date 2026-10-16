//! 匿名產品事件與獨立管理稽核契約；ProductEvent 不包含訊息本文、路徑或識別個資。
//! 對應參考 persisted_message_event／succeeded_event 的四欄角色，
//! 保留已驗證的來源／事件名稱白名單，不因參考命名不同改寫既有資料。
/// 獨立管理稽核契約；ProductEvent 本身仍維持匿名四欄。
pub mod audit;
pub mod topics;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductEvent {
    pub schema_version: u16,
    pub event_id: Uuid,
    pub source_app: String,
    pub event_type: String,
}

impl ProductEvent {
    pub fn chat_message_persisted(message_id: Uuid) -> Self {
        Self {
            schema_version: 1,
            event_id: message_id,
            source_app: "web_chat".into(),
            event_type: "message.persisted".into(),
        }
    }

    pub fn pet_cleanup_succeeded(operation_id: Uuid) -> Self {
        Self {
            schema_version: 1,
            event_id: operation_id,
            source_app: "desktop_pet".into(),
            event_type: "file.cleanup.succeeded".into(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.schema_version == 1
            && !self.event_id.is_nil()
            && (matches!(
                (self.source_app.as_str(), self.event_type.as_str()),
                (
                    "web_chat",
                    "message.persisted"
                        | "room.created"
                        | "login.succeeded"
                        | "login.failed"
                        | "logout.succeeded"
                ) | (
                    "desktop_pet",
                    "file.cleanup.succeeded"
                        | "file.restore.succeeded"
                        | "question.answered"
                        | "learning.review.completed"
                        | "pet.archived"
                        | "pet.restored"
                )
            ) || (self.source_app == "web_chat"
                && self
                    .event_type
                    .strip_prefix("keyword.")
                    .is_some_and(|word| topics::VOCABULARY.contains(&word))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_event_has_only_four_non_sensitive_fields() {
        let id = Uuid::new_v4();
        let value = serde_json::to_value(ProductEvent::chat_message_persisted(id)).unwrap();
        assert_eq!(value.as_object().unwrap().len(), 4);
        assert_eq!(value["event_id"], id.to_string());
        assert!(value.get("content").is_none());
    }

    #[test]
    fn only_allow_known_sources_and_schema() {
        let mut event = ProductEvent::chat_message_persisted(Uuid::new_v4());
        assert!(event.is_valid());
        event.source_app = "unknown".into();
        assert!(!event.is_valid());
    }
    #[test]
    fn unknown_or_cross_source_event_types_are_rejected() {
        let mut event = ProductEvent::chat_message_persisted(Uuid::new_v4());
        event.event_type = "private text".into();
        assert!(!event.is_valid());
        event.event_type = "file.cleanup.succeeded".into();
        assert!(!event.is_valid());
        event.source_app = "desktop_pet".into();
        assert!(event.is_valid());
        event.event_id = Uuid::nil();
        assert!(!event.is_valid());
    }
}
