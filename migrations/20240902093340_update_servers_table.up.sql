-- Add up migration script here
ALTER TABLE servers
ADD COLUMN rules_channel_id BIGINT;

ALTER TABLE servers
ADD COLUMN general_channel_id BIGINT;

ALTER TABLE servers
ADD COLUMN spoiler_channel_id BIGINT;

ALTER TABLE servers
ADD COLUMN support_channel_id BIGINT;

ALTER TABLE servers
ADD COLUMN suggestions_channel_id BIGINT;

ALTER TABLE servers
ADD COLUMN support_role_id BIGINT;

ALTER TABLE servers
ADD COLUMN artist_role_id BIGINT;

ALTER TABLE servers
ADD COLUMN sleep_role_id BIGINT;
