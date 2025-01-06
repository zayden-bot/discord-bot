-- Add up migration script here
CREATE TABLE lfg_users (
    id BIGINT PRIMARY KEY,
    timezone TEXT NOT NULL
);