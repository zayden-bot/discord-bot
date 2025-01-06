-- Add up migration script here
CREATE TABLE destiny_weapons (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    column_1 BIGINT[] NOT NULL DEFAULT '{}',
    column_2 BIGINT[] NOT NULL DEFAULT '{}',
    perk_1 BIGINT[] NOT NULL DEFAULT '{}',
    perk_2 BIGINT[] NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_destiny_weapons_name ON destiny_weapons (name);