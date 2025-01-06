-- Add up migration script here
CREATE TABLE lfg_posts (
    id BIGINT PRIMARY KEY NOT NULL,
    owner_id BIGINT NOT NULL,
    activity TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    timezone TEXT NOT NULL,
    description TEXT NOT NULL,
    fireteam_size SMALLINT NOT NULL,
    fireteam BIGINT[] NOT NULL,
    alternatives BIGINT[] NOT NULL
)
