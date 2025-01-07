-- Add up migration script here
ALTER TABLE guilds
ADD COLUMN thread_id INTEGER NOT NULL DEFAULT 0,
ADD COLUMN support_channel_id BIGINT,
ADD COLUMN support_role_ids BIGINT[] NOT NULL DEFAULT '{}',
ADD COLUMN faq_channel_id BIGINT;
