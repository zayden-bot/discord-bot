-- Add up migration script here

CREATE TABLE IF NOT EXISTS levels ( 
    id BIGINT PRIMARY KEY,
    total_xp INT NOT NULL DEFAULT 0,
    last_xp TIMESTAMP NOT NULL DEFAULT to_timestamp(0)
);