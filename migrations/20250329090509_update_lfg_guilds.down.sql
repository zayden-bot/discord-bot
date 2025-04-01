-- Add down migration script here
ALTER TABLE lfg_guilds
DROP COLUMN scheduled_thread_id;