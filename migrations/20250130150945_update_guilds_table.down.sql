-- Add down migration script here
ALTER TABLE guilds
DROP COLUMN xp_blocked_channels;
