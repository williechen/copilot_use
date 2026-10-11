CREATE TABLE chat_connections(id UUID PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),user_id UUID REFERENCES chat_users(id),expires_at TIMESTAMPTZ NOT NULL);
CREATE INDEX chat_connections_room_expiry ON chat_connections(room_id,expires_at);
