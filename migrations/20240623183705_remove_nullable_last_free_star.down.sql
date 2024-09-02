-- Add down migration script here
ALTER TABLE gold_stars
ALTER COLUMN last_free_star SET NULL;