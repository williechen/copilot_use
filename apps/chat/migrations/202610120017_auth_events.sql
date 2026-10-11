ALTER TABLE chat_outbox ALTER COLUMN message_id DROP NOT NULL;
ALTER TABLE chat_outbox DROP CONSTRAINT chat_outbox_event_type_check;
ALTER TABLE chat_outbox ADD CONSTRAINT chat_outbox_event_type_check CHECK(event_type IN ('message.persisted','login.succeeded','logout.succeeded'));
