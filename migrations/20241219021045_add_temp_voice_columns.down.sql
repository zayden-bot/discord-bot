-- Add down migration script here
ALTER TABLE guilds
DROP COLUMN temp_voice_category,
DROP COLUMN temp_voice_creator_channel;
