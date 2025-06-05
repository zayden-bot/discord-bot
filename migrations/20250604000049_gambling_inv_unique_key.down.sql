-- Add down migration script here
ALTER TABLE gambling_inventory
DROP CONSTRAINT uq_gambling_inventory_user_item;