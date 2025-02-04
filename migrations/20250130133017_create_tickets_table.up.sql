-- Add up migration script here
CREATE TABLE tickets (
    id BIGINT PRIMARY KEY,
    role_ids BIGINT[] NOT NULL DEFAULT '{}'
)