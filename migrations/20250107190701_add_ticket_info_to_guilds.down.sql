-- Add down migration script here
ALTER TABLE guilds
DROP COLUMN thread_id,
DROP COLUMN support_channel_id,
DROP COLUMN support_role_ids,
DROP COLUMN faq_channel_id;
