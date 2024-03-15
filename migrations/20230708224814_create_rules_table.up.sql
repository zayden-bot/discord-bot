-- Add up migration script here
CREATE TABLE IF NOT EXISTS server_rules (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    rule_id VARCHAR(255) NOT NULL,
    rule_text TEXT NOT NULL,
    is_hidden BOOLEAN NOT NULL DEFAULT FALSE
)