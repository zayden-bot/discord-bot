-- Add up migration script here
ALTER TABLE family ADD COLUMN IF NOT EXISTS blocked_ids BIGINT[] NOT NULL DEFAULT '{}';