use chat_application::{ChatService, product_event::persisted_message_event};

#[test]
fn mapper_emits_schema_source_and_kind() {
    let message = ChatService::new().send_message("private-room".into(), "private-content".into());
    let event = persisted_message_event(&message).unwrap();
    assert_eq!(event.schema_version, 1);
    assert_eq!(event.source_app, "web_chat");
    assert_eq!(event.event_type, "message.persisted");
    assert_eq!(event.event_id.to_string(), message.id);
}

#[test]
fn mapper_is_stable_and_omits_private_content_and_room() {
    let message = ChatService::new().send_message("private-room".into(), "private-content".into());
    let first = persisted_message_event(&message).unwrap();
    assert_eq!(first, persisted_message_event(&message).unwrap());
    let debug = format!("{first:?}");
    assert!(!debug.contains("private-room"));
    assert!(!debug.contains("private-content"));
}

#[test]
fn invalid_message_id_is_rejected_without_random_replacement() {
    let mut message = ChatService::new().send_message("room".into(), "content".into());
    message.id = "invalid-id".into();
    assert!(persisted_message_event(&message).is_err());
}
