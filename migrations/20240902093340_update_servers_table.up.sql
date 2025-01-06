-- Add up migration script here
ALTER TABLE servers
ADD COLUMN IF NOT EXISTS rules_channel_id BIGINT,
ADD COLUMN IF NOT EXISTS general_channel_id BIGINT,
ADD COLUMN IF NOT EXISTS spoiler_channel_id BIGINT,
ADD COLUMN IF NOT EXISTS support_channel_id BIGINT,
ADD COLUMN IF NOT EXISTS suggestions_channel_id BIGINT,
ADD COLUMN IF NOT EXISTS support_role_id BIGINT,
ADD COLUMN IF NOT EXISTS artist_role_id BIGINT,
ADD COLUMN IF NOT EXISTS sleep_role_id BIGINT;
