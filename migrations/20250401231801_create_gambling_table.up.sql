-- Add up migration script here
CREATE TABLE gambling (
    id BIGINT PRIMARY KEY,
    cash BIGINT NOT NULL DEFAULT 1000
);