-- Add up migration script here
ALTER TABLE family ADD COLUMN blocked_ids BIGINT[] NOT NULL DEFAULT '{}';