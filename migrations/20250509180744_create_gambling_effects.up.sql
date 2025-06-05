-- Add up migration script here
CREATE TABLE gambling_effects(
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    item_id TEXT NOT NULL,
    expiry TIMESTAMP
);