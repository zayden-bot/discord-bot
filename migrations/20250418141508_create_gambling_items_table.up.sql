-- Add up migration script here
CREATE TABLE IF NOT EXISTS gambling_inventory (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    item_id TEXT NOT NULL,
    quantity BIGINT NOT NULL DEFAULT 0,

    CONSTRAINT fk_inventory_user
        FOREIGN KEY(user_id)
        REFERENCES gambling(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_gambling_inventory_user_id ON gambling_inventory (user_id);