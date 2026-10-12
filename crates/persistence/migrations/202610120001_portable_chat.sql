-- 對照服務專用 DB；與 rust_ai_chat 的 Migration 完全隔離。
CREATE TABLE chat_rooms (
    id VARCHAR(40) PRIMARY KEY,
    name VARCHAR(200) NOT NULL
);
CREATE TABLE chat_messages (
    id BLOB PRIMARY KEY,
    room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),
    author_kind VARCHAR(32) NOT NULL DEFAULT 'anonymous_user',
    author_id VARCHAR(255),
    content TEXT NOT NULL CHECK(length(trim(content)) > 0),
    created_at REAL NOT NULL DEFAULT (unixepoch('subsec')),
    CHECK (author_kind IN ('anonymous_user','user','ai','system')),
    CHECK ((author_kind IN ('anonymous_user','system') AND author_id IS NULL)
        OR (author_kind IN ('user','ai') AND author_id IS NOT NULL))
);
CREATE TABLE chat_outbox (
    event_id BLOB PRIMARY KEY,
    message_id BLOB UNIQUE REFERENCES chat_messages(id),
    schema_version SMALLINT NOT NULL CHECK(schema_version=1),
    source_app VARCHAR(40) NOT NULL CHECK(source_app='web_chat'),
    event_type VARCHAR(80) NOT NULL CHECK(event_type IN ('message.persisted','login.succeeded','login.failed','logout.succeeded','keyword.rust','keyword.ownership','keyword.borrow','keyword.lifetime','keyword.async','keyword.trait','keyword.sql','keyword.postgresql','keyword.leptos','keyword.tauri')),
    status VARCHAR(16) NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending','leased','delivered','blocked')),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK(attempts >= 0),
    next_attempt_at REAL NOT NULL DEFAULT (unixepoch('subsec')),
    lease_token BLOB,
    lease_until REAL,
    delivered_at REAL,
    last_error VARCHAR(128),
    created_at REAL NOT NULL DEFAULT (unixepoch('subsec')),
    CHECK ((status='leased' AND lease_token IS NOT NULL AND lease_until IS NOT NULL)
        OR (status<>'leased' AND lease_token IS NULL AND lease_until IS NULL))
);
CREATE INDEX chat_outbox_due_idx ON chat_outbox (next_attempt_at, created_at)
 WHERE status IN ('pending','leased');
CREATE TABLE chat_users(id BLOB PRIMARY KEY,username VARCHAR(100) NOT NULL UNIQUE,password_hash TEXT NOT NULL);
CREATE TABLE chat_sessions(token_hash BLOB PRIMARY KEY,user_id BLOB NOT NULL REFERENCES chat_users(id) ON DELETE CASCADE,expires_at REAL NOT NULL);
CREATE INDEX chat_sessions_expiry ON chat_sessions(expires_at);
CREATE TABLE chat_room_members(room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,user_id BLOB NOT NULL REFERENCES chat_users(id) ON DELETE CASCADE,PRIMARY KEY(room_id,user_id));
ALTER TABLE chat_rooms ADD COLUMN owner_id BLOB REFERENCES chat_users(id);


CREATE TABLE knowledge_documents(id BLOB PRIMARY KEY,owner_id BLOB REFERENCES chat_users(id) ON DELETE CASCADE,title VARCHAR(200) NOT NULL,content TEXT NOT NULL,embedding TEXT NOT NULL CHECK(json_valid(embedding) AND json_array_length(embedding)=1536),created_at REAL NOT NULL DEFAULT (unixepoch('subsec')));
CREATE INDEX knowledge_owner ON knowledge_documents(owner_id);

ALTER TABLE chat_outbox ADD COLUMN traceparent VARCHAR(55);

CREATE TABLE chat_room_policy(room_id VARCHAR(40) PRIMARY KEY REFERENCES chat_rooms(id),context_messages BIGINT NOT NULL DEFAULT 30 CHECK(context_messages BETWEEN 10 AND 100),revision BIGINT NOT NULL DEFAULT 0);
CREATE TABLE chat_suggestion_receipts(proposal_id BLOB NOT NULL,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),before_value BIGINT NOT NULL,after_value BIGINT NOT NULL,applied_revision BIGINT NOT NULL,applied BOOLEAN NOT NULL DEFAULT TRUE,apply_receipt BLOB NOT NULL,rollback_receipt BLOB,PRIMARY KEY(proposal_id,room_id));

CREATE TABLE chat_connections(id BLOB PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),user_id BLOB REFERENCES chat_users(id),expires_at REAL NOT NULL);
CREATE INDEX chat_connections_room_expiry ON chat_connections(room_id,expires_at);





CREATE TABLE chat_personas(id BLOB PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),name VARCHAR(80) NOT NULL,instructions TEXT NOT NULL CHECK(length(CAST(instructions AS BLOB))<=4096),memory TEXT NOT NULL DEFAULT '' CHECK(length(CAST(memory AS BLOB))<=2048),cooldown_seconds INTEGER NOT NULL DEFAULT 2 CHECK(cooldown_seconds BETWEEN 1 AND 30),enabled BOOLEAN NOT NULL DEFAULT true,revision BIGINT NOT NULL DEFAULT 0,last_spoke_at REAL,created_at REAL NOT NULL DEFAULT (unixepoch('subsec')));
CREATE INDEX chat_personas_room ON chat_personas(room_id,created_at,id);
CREATE TABLE chat_persona_runs(id BLOB PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),parameters TEXT NOT NULL,status TEXT NOT NULL CHECK(status IN ('running','completed','failed','cancelled','budget_exhausted')),max_turns INTEGER NOT NULL CHECK(max_turns BETWEEN 1 AND 8),max_output_budget INTEGER NOT NULL CHECK(max_output_budget BETWEEN 512 AND 16384),reserved_tokens INTEGER NOT NULL DEFAULT 0,result TEXT NOT NULL DEFAULT '[]',updated_at REAL NOT NULL DEFAULT (unixepoch('subsec')));
CREATE TABLE chat_ai_room_locks(room_id VARCHAR(40) PRIMARY KEY REFERENCES chat_rooms(id),lease_token BLOB NOT NULL,expires_at REAL NOT NULL);

ALTER TABLE chat_persona_runs ADD COLUMN failure_status INTEGER;

CREATE TABLE chat_auth_throttle(key TEXT PRIMARY KEY,attempts INTEGER NOT NULL,expires_at REAL NOT NULL);
CREATE INDEX chat_auth_throttle_expiry ON chat_auth_throttle(expires_at);



CREATE TABLE chat_topic_consent(user_id BLOB NOT NULL REFERENCES chat_users(id),room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),enabled BOOLEAN NOT NULL DEFAULT false,withdraw_before REAL,withdraw_pending BOOLEAN NOT NULL DEFAULT false,PRIMARY KEY(user_id,room_id));
CREATE TABLE chat_topic_events(event_id BLOB PRIMARY KEY,message_id BLOB NOT NULL REFERENCES chat_messages(id),user_id BLOB NOT NULL REFERENCES chat_users(id),room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),created_at REAL NOT NULL DEFAULT (unixepoch('subsec')),withdrawal_queued BOOLEAN NOT NULL DEFAULT false,withdrawal_ack BOOLEAN NOT NULL DEFAULT false);
CREATE INDEX chat_topic_events_owner_idx ON chat_topic_events(user_id,room_id,created_at) WHERE NOT withdrawal_ack;
CREATE TABLE chat_topic_withdraw_outbox(id BLOB PRIMARY KEY,event_ids TEXT NOT NULL CHECK(json_valid(event_ids) AND json_array_length(event_ids) BETWEEN 1 AND 500),status VARCHAR(16) NOT NULL DEFAULT 'pending' CHECK(status IN ('pending','leased','delivered')),attempts INTEGER NOT NULL DEFAULT 0,next_attempt_at REAL NOT NULL DEFAULT (unixepoch('subsec')),lease_token BLOB,lease_until REAL);

-- Persistent process-neutral notifications, never uncommitted message content.
CREATE TABLE chat_realtime(seq INTEGER PRIMARY KEY AUTOINCREMENT,payload TEXT NOT NULL CHECK(json_valid(payload)),created_at REAL NOT NULL DEFAULT(unixepoch('subsec')));
CREATE INDEX chat_realtime_created ON chat_realtime(created_at);
