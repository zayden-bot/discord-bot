-- Add up migration script here
ALTER TABLE family ADD COLUMN username VARCHAR;
UPDATE family SET username = 'NULL';
ALTER TABLE family ALTER COLUMN username SET NOT NULL;