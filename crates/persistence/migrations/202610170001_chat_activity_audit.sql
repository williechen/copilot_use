-- 私有操作稽核；不回填無法還原使用者的歷史匿名事件。
CREATE TABLE chat_activity_audit (
    event_id BLOB PRIMARY KEY,
    event_type TEXT NOT NULL CHECK(event_type IN ('login.succeeded','login.failed','logout.succeeded','room.created')),
    user_id BLOB,
    username TEXT NOT NULL CHECK(length(CAST(username AS BLOB))<=100),
    room_id TEXT,
    room_name TEXT,
    target_username TEXT,
    occurred_unix_ms INTEGER NOT NULL DEFAULT (CAST(unixepoch('subsec')*1000 AS INTEGER))
);
CREATE INDEX chat_activity_audit_time ON chat_activity_audit(occurred_unix_ms DESC,event_id);
CREATE INDEX chat_activity_audit_user ON chat_activity_audit(username,occurred_unix_ms DESC);

-- SQLite 以新表擴充 CHECK 白名單；同一 migration 交易逐欄保留所有事件及交付狀態。
CREATE TABLE chat_outbox_expanded (
    event_id BLOB PRIMARY KEY,
    message_id BLOB UNIQUE REFERENCES chat_messages(id),
    schema_version SMALLINT NOT NULL CHECK(schema_version=1),
    source_app VARCHAR(40) NOT NULL CHECK(source_app='web_chat'),
    event_type VARCHAR(80) NOT NULL CHECK(event_type IN ('message.persisted','room.created','login.succeeded','login.failed','logout.succeeded','keyword.rust','keyword.ownership','keyword.borrow','keyword.lifetime','keyword.async','keyword.trait','keyword.sql','keyword.postgresql','keyword.leptos','keyword.tauri')),
    status VARCHAR(16) NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending','leased','delivered','blocked')),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK(attempts >= 0),
    next_attempt_at REAL NOT NULL DEFAULT (unixepoch('subsec')),
    lease_token BLOB,
    lease_until REAL,
    delivered_at REAL,
    last_error VARCHAR(128),
    created_at REAL NOT NULL DEFAULT (unixepoch('subsec')),
    traceparent VARCHAR(55),
    CHECK ((status='leased' AND lease_token IS NOT NULL AND lease_until IS NOT NULL)
        OR (status<>'leased' AND lease_token IS NULL AND lease_until IS NULL))
);
INSERT INTO chat_outbox_expanded(event_id,message_id,schema_version,source_app,event_type,status,attempts,next_attempt_at,lease_token,lease_until,delivered_at,last_error,created_at,traceparent) SELECT event_id,message_id,schema_version,source_app,event_type,status,attempts,next_attempt_at,lease_token,lease_until,delivered_at,last_error,created_at,traceparent FROM chat_outbox;
DROP TABLE chat_outbox;
ALTER TABLE chat_outbox_expanded RENAME TO chat_outbox;
CREATE INDEX chat_outbox_due_idx ON chat_outbox(next_attempt_at,created_at) WHERE status IN ('pending','leased');

-- 舊房間保留原私聊語意；新建房間由 API 明確寫入預設公開。
ALTER TABLE chat_rooms ADD COLUMN is_private BOOLEAN NOT NULL DEFAULT true CHECK(is_private IN (0,1));
