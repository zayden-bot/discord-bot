-- Add up migration script here
ALTER TABLE gambling
ADD COLUMN game TIMESTAMP NOT NULL DEFAULT '1970-01-01';