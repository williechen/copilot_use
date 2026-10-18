-- Forward-only repair. Preserve valid rows; NULL legacy IDs abort the migration
-- transaction so an operator can repair identity without silently losing logs.
CREATE TABLE chat_action_log_required_id (
    id TEXT NOT NULL PRIMARY KEY,
    action_type VARCHAR(255) NOT NULL,
    action_content TEXT,
    ref_table VARCHAR(255),
    ref_id TEXT,
    ref_column VARCHAR(255),
    created_user_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO chat_action_log_required_id
    (id, action_type, action_content, ref_table, ref_id, ref_column, created_user_id, created_at)
SELECT id, action_type, action_content, ref_table, ref_id, ref_column, created_user_id, created_at
FROM chat_action_log;
DROP TABLE chat_action_log;
ALTER TABLE chat_action_log_required_id RENAME TO chat_action_log;
