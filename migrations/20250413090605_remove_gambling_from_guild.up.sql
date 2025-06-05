-- Add up migration script here
ALTER TABLE guilds
DROP COLUMN gambling_lost,
DROP COLUMN gambling_gain;