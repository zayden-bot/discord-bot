-- Add up migration script here
CREATE TABLE lfg_guilds (
    id BIGINT PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    role_id BIGINT
);