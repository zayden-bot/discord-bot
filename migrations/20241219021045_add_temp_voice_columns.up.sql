-- Add up migration script here
ALTER TABLE guilds
ADD COLUMN temp_voice_category BIGINT,
ADD COLUMN temp_voice_creator_channel BIGINT;