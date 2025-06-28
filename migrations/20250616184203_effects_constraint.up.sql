-- Add up migration script here
ALTER TABLE gambling_effects
ADD CONSTRAINT unique_user_item UNIQUE (user_id, item_id);