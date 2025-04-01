-- Add up migration script here
ALTER TABLE lfg_guilds
ADD COLUMN scheduled_thread_id BIGINT;