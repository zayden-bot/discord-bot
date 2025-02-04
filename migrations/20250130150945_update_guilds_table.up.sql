-- Add up migration script here
ALTER TABLE guilds
ADD COLUMN xp_blocked_channels BIGINT[] NOT NULL DEFAULT '{}';