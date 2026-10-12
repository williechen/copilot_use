ALTER TABLE product_events ADD COLUMN consent_withdrawn BOOLEAN NOT NULL DEFAULT false;
CREATE TABLE topic_privacy_state(event_id UUID PRIMARY KEY,withdrawn BOOLEAN NOT NULL DEFAULT false);
