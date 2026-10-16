//! 管理者專用操作稽核契約；與匿名 ProductEvent 分開，不提供給聚合分析或 AI。
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuditQuery {
    pub username: Option<String>,
    pub event_type: Option<String>,
    pub succeeded: Option<bool>,
    pub from_unix_ms: Option<i64>,
    pub to_unix_ms: Option<i64>,
}
impl AuditQuery {
    pub fn is_valid(&self) -> bool {
        self.username
            .as_ref()
            .is_none_or(|v| !v.is_empty() && v.len() <= 100)
            && self.event_type.as_deref().is_none_or(valid_kind)
            && self.from_unix_ms.is_none_or(|v| v >= 0)
            && self.to_unix_ms.is_none_or(|v| v >= 0)
            && match (self.from_unix_ms, self.to_unix_ms) {
                (Some(a), Some(b)) => a < b && b - a <= 366 * 86400000,
                _ => true,
            }
    }
}
pub fn valid_kind(kind: &str) -> bool {
    matches!(
        kind,
        "login.succeeded" | "login.failed" | "logout.succeeded" | "room.created"
    )
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuditRecord {
    pub event_id: Uuid,
    pub event_type: String,
    pub user_id: Option<Uuid>,
    pub username: String,
    pub room_id: Option<String>,
    pub room_name: Option<String>,
    pub target_username: Option<String>,
    pub occurred_unix_ms: i64,
    pub succeeded: bool,
}
