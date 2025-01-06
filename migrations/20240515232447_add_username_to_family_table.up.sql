-- Add up migration script here
ALTER TABLE family
    ADD COLUMN IF NOT EXISTS username VARCHAR;

UPDATE family
    SET username = 'NULL'
    WHERE username IS NULL;

ALTER TABLE family
    ALTER COLUMN username SET NOT NULL;