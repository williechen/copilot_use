ALTER TABLE conversations ADD COLUMN ordinal INTEGER NOT NULL DEFAULT 0;
UPDATE conversations SET ordinal=rowid;
CREATE UNIQUE INDEX conversations_pet_ordinal ON conversations(pet_id,ordinal);
