-- 對照服務專用 DB；與 rust_ai_chat 的 Migration 完全隔離。
CREATE TABLE chat_rooms (
    id VARCHAR(40) PRIMARY KEY,
    name VARCHAR(200) NOT NULL
);
CREATE TABLE chat_messages (
    id UUID PRIMARY KEY,
    room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),
    author_kind VARCHAR(32) NOT NULL DEFAULT 'anonymous_user',
    author_id VARCHAR(255),
    content TEXT NOT NULL CHECK(length(trim(content)) > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (author_kind IN ('anonymous_user','user','ai','system')),
    CHECK ((author_kind IN ('anonymous_user','system') AND author_id IS NULL)
        OR (author_kind IN ('user','ai') AND author_id IS NOT NULL))
);
CREATE TABLE chat_outbox (
    event_id UUID PRIMARY KEY,
    message_id UUID NOT NULL UNIQUE REFERENCES chat_messages(id),
    schema_version SMALLINT NOT NULL CHECK(schema_version=1),
    source_app VARCHAR(40) NOT NULL CHECK(source_app='web_chat'),
    event_type VARCHAR(80) NOT NULL CHECK(event_type='message.persisted'),
    status VARCHAR(16) NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending','leased','delivered','blocked')),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK(attempts >= 0),
    next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    lease_token UUID,
    lease_until TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    last_error VARCHAR(128),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((status='leased' AND lease_token IS NOT NULL AND lease_until IS NOT NULL)
        OR (status<>'leased' AND lease_token IS NULL AND lease_until IS NULL))
);
CREATE INDEX chat_outbox_due_idx ON chat_outbox (next_attempt_at, created_at)
 WHERE status IN ('pending','leased');