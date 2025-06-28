-- Add down migration script here
ALTER TABLE gambling_effects
DROP CONSTRAINT unique_user_item;