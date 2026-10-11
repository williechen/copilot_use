-- Analytics 使用自己的 PostgreSQL database；與 Chat 的資料庫分離。
CREATE TABLE IF NOT EXISTS product_events (
    source_app VARCHAR(40) NOT NULL
        CHECK (source_app IN ('web_chat','desktop_pet')),
    event_id UUID NOT NULL,
    schema_version INTEGER NOT NULL CHECK(schema_version=1),
    event_type VARCHAR(128) NOT NULL CHECK(length(event_type)>0),
    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (source_app, event_id)
);
CREATE INDEX IF NOT EXISTS idx_product_events_received_at ON product_events(received_at);
