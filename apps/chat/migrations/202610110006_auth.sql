CREATE TABLE chat_users(id UUID PRIMARY KEY,username VARCHAR(100) NOT NULL UNIQUE,password_hash TEXT NOT NULL);
CREATE TABLE chat_sessions(token_hash BYTEA PRIMARY KEY,user_id UUID NOT NULL REFERENCES chat_users(id) ON DELETE CASCADE,expires_at TIMESTAMPTZ NOT NULL);
CREATE INDEX chat_sessions_expiry ON chat_sessions(expires_at);
CREATE TABLE chat_room_members(room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,user_id UUID NOT NULL REFERENCES chat_users(id) ON DELETE CASCADE,PRIMARY KEY(room_id,user_id));
ALTER TABLE chat_rooms ADD COLUMN owner_id UUID REFERENCES chat_users(id);
