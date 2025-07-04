-- Add up migration script here
ALTER TABLE gambling
ADD CONSTRAINT coins_must_be_non_negative
CHECK (coins >= 0);