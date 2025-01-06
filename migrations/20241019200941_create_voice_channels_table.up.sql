-- Add up migration script here
CREATE TABLE voice_channels (
    id BIGINT PRIMARY KEY NOT NULL,
    owner_id BIGINT NOT NULL,
    trusted_ids BIGINT[] NOT NULL DEFAULT '{}',
    password TEXT DEFAULT NULL,
    persistent BOOLEAN NOT NULL DEFAULT FALSE
)