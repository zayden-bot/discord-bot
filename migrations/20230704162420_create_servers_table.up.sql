-- Add up migration script here
CREATE TABLE IF NOT EXISTS servers (
     id BIGINT PRIMARY KEY,
     support_thread_id INTEGER NOT NULL DEFAULT 0
);
