-- Add up migration script here
CREATE TABLE lfg_posts (
    id BIGINT PRIMARY KEY,
    "owner" BIGINT NOT NULL,
    activity TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    "description" TEXT NOT NULL DEFAULT '',
    fireteam_size SMALLINT NOT NULL
);

CREATE INDEX idx_lfg_posts_owner_id ON lfg_posts("owner");


CREATE TABLE lfg_fireteam (
    post BIGINT NOT NULL REFERENCES lfg_posts(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL,
    PRIMARY KEY (post, user_id)
);


CREATE TABLE lfg_alternatives (
    post BIGINT NOT NULL REFERENCES lfg_posts(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL,
    PRIMARY KEY (post, user_id)
);


CREATE TABLE lfg_messages (
    id BIGINT PRIMARY KEY REFERENCES lfg_posts(id) ON DELETE CASCADE,
    "message" BIGINT NOT NULL,
    channel BIGINT NOT NULL
);
