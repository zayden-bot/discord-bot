-- Add up migration script here
CREATE TABLE support_faq (
    id VARCHAR(255) PRIMARY KEY,
    answer TEXT NOT NULL,
    guild_id BIGINT NOT NULL
)