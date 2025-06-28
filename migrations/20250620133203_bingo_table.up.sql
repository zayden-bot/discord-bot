-- Add up migration script here
CREATE TABLE bingo(
    id BIGINT PRIMARY KEY,
    day DATE NOT NULL,
    spaces TEXT[] NOT NULL
);