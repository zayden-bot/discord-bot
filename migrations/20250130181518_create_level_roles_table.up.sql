-- Add up migration script here
CREATE TABLE level_roles (
    id BIGINT PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    level INT NOT NULL
);