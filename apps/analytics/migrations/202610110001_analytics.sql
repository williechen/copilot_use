-- Analytics 必須使用與 Chat 不同的 PostgreSQL database/角色。
CREATE TABLE product_events (
    source_app VARCHAR(40) NOT NULL CHECK(source_app IN ('web_chat','desktop_pet')),
    event_id UUID NOT NULL,
    schema_version SMALLINT NOT NULL CHECK(schema_version=1),
    event_type VARCHAR(128) NOT NULL CHECK(length(trim(event_type)) > 0),
    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY(source_app, event_id)
);
CREATE INDEX product_events_received_idx ON product_events (received_at);