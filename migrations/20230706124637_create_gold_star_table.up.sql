-- Add up migration script here
CREATE TABLE IF NOT EXISTS gold_stars (
    id BIGINT PRIMARY KEY,
    number_of_stars INT NOT NULL DEFAULT 0,
    given_stars INT NOT NULL DEFAULT 0,
    received_stars INT NOT NULL DEFAULT 0,
    last_free_star TIMESTAMP
)