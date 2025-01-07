-- Add down migration script here
ALTER TABLE guilds
DROP COLUMN IF EXISTS support_role_ids;
