-- Add down migration script here
ALTER TABLE voice_channels
DROP COLUMN invites,
DROP COLUMN mode;

DROP TYPE temp_voice_mode;
