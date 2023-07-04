-- Add up migration script here
CREATE TABLE servers (
     id BIGINT PRIMARY KEY,
     support_thread_id INTEGER NOT NULL DEFAULT 0
);
