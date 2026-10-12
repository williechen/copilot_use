ALTER TABLE analysis_suggestions DROP CONSTRAINT analysis_suggestions_status_check;
ALTER TABLE analysis_suggestions ADD CONSTRAINT analysis_suggestions_status_check CHECK(status IN ('pending','approved','applied','rejected','rolled_back'));
ALTER TABLE analysis_suggestions ADD COLUMN receipt_id UUID;
ALTER TABLE analysis_suggestions ADD COLUMN evaluation JSONB;
