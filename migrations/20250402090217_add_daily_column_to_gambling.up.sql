-- Add up migration script here
ALTER TABLE gambling
ADD COLUMN daily DATE NOT NULL DEFAULT '1970-01-01';