-- Add up migration script here
ALTER TABLE guilds
ADD COLUMN IF NOT EXISTS support_role_ids BIGINT[];