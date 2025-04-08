-- Add down migration script here
ALTER TABLE gambling
DROP COLUMN work,
DROP COLUMN gift;