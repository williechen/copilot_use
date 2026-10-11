//! Day 9 跨系統最小事件契約；不包含訊息本文、路徑或識別個資。
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
            && matches!(self.source_app.as_str(), "web_chat" | "desktop_pet")
            && !self.event_type.trim().is_empty()
            && self.event_type.len() <= 128
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
}
