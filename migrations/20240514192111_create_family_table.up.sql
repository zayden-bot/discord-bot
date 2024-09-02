-- Add up migration script here
CREATE TABLE IF NOT EXISTS family (
    id BIGINT PRIMARY KEY,
    partner_ids BIGINT[] NOT NULL DEFAULT '{}',
    parent_ids BIGINT[] NOT NULL DEFAULT '{}',
    children_ids BIGINT[] NOT NULL DEFAULT '{}'
)