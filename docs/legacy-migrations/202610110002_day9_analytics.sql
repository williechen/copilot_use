-- Analytics 獨立 PostgreSQL 資料庫。不要在 Chat DB 套用此 migration。
CREATE TABLE product_events (
    source_app TEXT NOT NULL,
    event_id UUID NOT NULL,
    schema_version SMALLINT NOT NULL CHECK (schema_version = 1),
    event_type TEXT NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (source_app, event_id)
);
CREATE INDEX product_events_received_at_idx ON product_events(received_at);
