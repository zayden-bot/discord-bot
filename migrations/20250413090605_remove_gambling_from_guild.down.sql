-- Add down migration script here
ALTER TABLE guilds
ADD COLUMN gambling_lost BIGINT NOT NULL DEFAULT 0,
ADD COLUMN gambling_gain BIGINT NOT NULL DEFAULT 0;