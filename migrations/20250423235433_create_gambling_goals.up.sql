-- Add up migration script here
CREATE TABLE gambling_goals(
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    goal_id TEXT NOT NULL,
    day date NOT NULL DEFAULT '1970-01-01',
    progress BIGINT NOT NULL DEFAULT 0,
    target BIGINT NOT NULL DEFAULT 1
);

CREATE INDEX idx_gambling_goals_user_id ON gambling_inventory (user_id);