-- Add up migration script here
CREATE TYPE temp_voice_mode AS ENUM ('open', 'spectator', 'locked', 'invisible');

ALTER TABLE voice_channels
ADD COLUMN invites BIGINT[] NOT NULL DEFAULT '{}',
ADD COLUMN mode temp_voice_mode NOT NULL DEFAULT 'open';