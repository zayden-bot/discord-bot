-- Add up migration script here
ALTER TABLE guilds
ADD COLUMN suggestions_channel_id BIGINT,
ADD COLUMN review_channel_id BIGINT;