-- Day9 的獨立測試資料庫；不修改 rust_ai_chat 既有 migrations。
CREATE TABLE IF NOT EXISTS chat_rooms (
    id VARCHAR(40) PRIMARY KEY
);
CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY,
    room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),
    author_kind VARCHAR(32) NOT NULL
      CHECK(author_kind IN ('anonymous_user','user','ai','system')),
    author_id VARCHAR(255),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT author_identity CHECK (
        (author_kind IN ('anonymous_user','system') AND author_id IS NULL)
        OR (author_kind IN ('user','ai') AND author_id IS NOT NULL)
    )
);
CREATE TABLE IF NOT EXISTS chat_outbox (
    event_id UUID PRIMARY KEY,
    message_id UUID NOT NULL UNIQUE REFERENCES chat_messages(id),
    schema_version INTEGER NOT NULL CHECK (schema_version=1),
    source_app VARCHAR(40) NOT NULL CHECK (source_app='web_chat'),
    event_type VARCHAR(80) NOT NULL CHECK (event_type='message.persisted'),
    status VARCHAR(16) NOT NULL DEFAULT 'pending'
      CHECK (status IN ('pending','leased','delivered','blocked')),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK(attempts>=0),
    next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    lease_id UUID,
    leased_until TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (
      (status='leased' AND lease_id IS NOT NULL AND leased_until IS NOT NULL)
      OR (status<>'leased' AND lease_id IS NULL AND leased_until IS NULL)
    )
);
CREATE INDEX IF NOT EXISTS idx_chat_outbox_due
 ON chat_outbox (next_attempt_at, created_at)
 WHERE status IN ('pending','leased');
