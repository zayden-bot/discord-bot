-- Add up migration script here
UPDATE gold_stars
SET last_free_star = '1970-01-01 00:00:00'
WHERE last_free_star IS NULL;

ALTER TABLE gold_stars
ALTER COLUMN last_free_star SET NOT NULL;