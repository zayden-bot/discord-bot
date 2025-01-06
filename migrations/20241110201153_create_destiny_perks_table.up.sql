-- Add up migration script here
CREATE TABLE destiny_perks (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE INDEX idx_destiny_perks_name ON destiny_perks (name);