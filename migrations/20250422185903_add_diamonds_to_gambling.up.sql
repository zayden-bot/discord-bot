-- Add up migration script here
ALTER TABLE gambling
ADD COLUMN diamonds INT NOT NULL DEFAULT 0;
