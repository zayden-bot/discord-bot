-- Add up migration script here
CREATE TABLE reaction_roles (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    message_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    emoji VARCHAR(255) NOT NULL
)