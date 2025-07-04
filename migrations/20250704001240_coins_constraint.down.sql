-- Add down migration script here
ALTER TABLE gambling
DROP CONSTRAINT coins_must_be_non_negative;