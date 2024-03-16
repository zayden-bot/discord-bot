-- Add up migration script here
ALTER TABLE levels ADD COLUMN message_count INT NOT NULL DEFAULT 0;