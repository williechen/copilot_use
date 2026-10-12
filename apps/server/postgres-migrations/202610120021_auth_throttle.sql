CREATE TABLE chat_auth_throttle(key TEXT PRIMARY KEY,attempts INTEGER NOT NULL,expires_at TIMESTAMPTZ NOT NULL);
CREATE INDEX chat_auth_throttle_expiry ON chat_auth_throttle(expires_at);
