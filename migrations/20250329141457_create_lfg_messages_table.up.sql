-- Add up migration script here
CREATE TABLE lfg_messages (
    id BIGINT PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    post_id BIGINT NOT NULL
);