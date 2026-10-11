CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE knowledge_documents(id UUID PRIMARY KEY,owner_id UUID REFERENCES chat_users(id) ON DELETE CASCADE,title VARCHAR(200) NOT NULL,content TEXT NOT NULL,embedding vector(1536) NOT NULL,created_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE INDEX knowledge_owner ON knowledge_documents(owner_id);
