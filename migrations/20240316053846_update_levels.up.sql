-- Add up migration script here
ALTER TABLE levels
ADD COLUMN IF NOT EXISTS message_count INT NOT NULL DEFAULT 0;