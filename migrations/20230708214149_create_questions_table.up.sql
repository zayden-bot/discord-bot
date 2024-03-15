-- Add up migration script here
CREATE TABLE IF NOT EXISTS questions (
    id SERIAL PRIMARY KEY,
    question TEXT NOT NULL,
    answer TEXT,
    user_id BIGINT NOT NULL,
    message_id BIGINT
)