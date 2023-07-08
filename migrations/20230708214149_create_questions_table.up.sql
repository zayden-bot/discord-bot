-- Add up migration script here
CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    question TEXT NOT NULL,
    answer TEXT,
    user_id BIGINT NOT NULL,
    message_id BIGINT
)