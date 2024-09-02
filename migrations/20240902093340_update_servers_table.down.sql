-- Add down migration script here
ALTER TABLE servers
DROP COLUMN rules_channel_id BIGINT;

ALTER TABLE servers
DROP COLUMN general_channel_id BIGINT;

ALTER TABLE servers
DROP COLUMN spoiler_channel_id BIGINT;

ALTER TABLE servers
DROP COLUMN support_channel_id BIGINT;

ALTER TABLE servers
DROP COLUMN suggestions_channel_id BIGINT;

ALTER TABLE servers
DROP COLUMN support_role_id BIGINT;

ALTER TABLE servers
DROP COLUMN artist_role_id BIGINT;

ALTER TABLE servers
DROP COLUMN sleep_role_id BIGINT;
