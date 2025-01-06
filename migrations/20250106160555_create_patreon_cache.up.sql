-- Add up migration script here
CREATE TABLE patreon_cache (
    email TEXT PRIMARY KEY,
    id TEXT NOT NULL,
    discord_id TEXT
);