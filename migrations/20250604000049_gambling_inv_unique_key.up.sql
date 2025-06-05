-- Add up migration script here
ALTER TABLE gambling_inventory
ADD CONSTRAINT uq_gambling_inventory_user_item UNIQUE (user_id, item_id);