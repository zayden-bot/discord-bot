-- Add down migration script here
ALTER TABLE gambling
RENAME COLUMN coins TO cash;

ALTER TABLE gambling
RENAME COLUMN gems TO diamonds;

ALTER TABLE gambling
RENAME COLUMN stamina TO work;

ALTER TABLE gambling
ALTER COLUMN work DROP DEFAULT;

ALTER TABLE gambling
ALTER COLUMN work TYPE TIMESTAMP USING '1970-01-01';

ALTER TABLE gambling
ALTER COLUMN work SET DEFAULT '1970-01-01';

CREATE TABLE level_roles (
    id BIGINT PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    level INT NOT NULL
);

CREATE TABLE patreon_cache (
    email TEXT PRIMARY KEY,
    id TEXT NOT NULL,
    discord_id BIGINT
);

CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    question TEXT NOT NULL,
    answer TEXT,
    user_id BIGINT NOT NULL,
    message_id BIGINT
);

CREATE TABLE server_rules (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    rule_id VARCHAR(255) NOT NULL,
    rule_text TEXT NOT NULL,
    is_hidden BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE support_faq (
    id VARCHAR(255) PRIMARY KEY,
    answer TEXT NOT NULL,
    guild_id BIGINT NOT NULL
);