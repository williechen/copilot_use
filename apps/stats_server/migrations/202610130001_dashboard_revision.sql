-- The revision changes in the SAME transaction as event data. Polling observes only commits.
CREATE TABLE analytics_revision (
    singleton boolean PRIMARY KEY DEFAULT true CHECK (singleton),
    revision bigint NOT NULL DEFAULT 0 CHECK (revision >= 0)
);
INSERT INTO analytics_revision(singleton) VALUES (true);
CREATE FUNCTION advance_analytics_revision() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    UPDATE analytics_revision SET revision = revision + 1 WHERE singleton;
    RETURN NULL;
END;
$$;
CREATE TRIGGER analytics_events_changed
AFTER INSERT OR UPDATE OR DELETE ON product_events
FOR EACH ROW EXECUTE FUNCTION advance_analytics_revision();
