-- 此為 copilot_use 獨立 PostgreSQL 對照環境；不可套用至 rust_ai_chat 原資料庫。
-- 本版先選 native UUID；切換 VARCHAR(40) 時僅變更 Migration 與 persistence adapter。
CREATE TABLE chat_messages (
    id UUID PRIMARY KEY,
    room_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE chat_outbox (
    event_id UUID PRIMARY KEY REFERENCES chat_messages(id),
    schema_version SMALLINT NOT NULL CHECK (schema_version = 1),
    source_app TEXT NOT NULL,
    event_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'leased', 'delivered', 'blocked')),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    lease_token UUID,
    lease_until TIMESTAMPTZ,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    delivered_at TIMESTAMPTZ
);
CREATE INDEX chat_outbox_due ON chat_outbox (next_attempt_at, created_at)
WHERE status IN ('pending', 'leased');
