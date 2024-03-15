-- Add up migration script here
CREATE TABLE IF NOT EXISTS infractions (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    username VARCHAR(255) NOT NULL,
    guild_id BIGINT NOT NULL,
    infraction_type VARCHAR(255) NOT NULL,
    moderator_id BIGINT NOT NULL,
    moderator_username VARCHAR(255) NOT NULL,
    points INT NOT NULL DEFAULT 1,
    reason VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)