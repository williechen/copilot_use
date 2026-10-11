ALTER TABLE pet_outbox ADD COLUMN lease_token TEXT;
ALTER TABLE pet_outbox ADD COLUMN lease_until INTEGER;
ALTER TABLE pet_outbox ADD COLUMN last_error TEXT;
ALTER TABLE pet_outbox ADD COLUMN traceparent TEXT;
CREATE INDEX pet_outbox_ready ON pet_outbox(delivered,next_attempt_at,lease_until);
