-- Add up migration script here
ALTER TABLE gambling_mine
ADD COLUMN mine_activity TIMESTAMP NOT NULL DEFAULT 'now()';