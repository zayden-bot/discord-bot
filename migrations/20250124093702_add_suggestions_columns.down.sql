-- Add down migration script here
ALTER TABLE guilds
DROP COLUMN suggestions_channel_id,
DROP COLUMN review_channel_id;
