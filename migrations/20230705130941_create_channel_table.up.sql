-- Add up migration script here
CREATE TABLE IF NOT EXISTS channels (
    id BIGINT PRIMARY KEY,
    category VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL,
    guild_id BIGINT NOT NULL
);