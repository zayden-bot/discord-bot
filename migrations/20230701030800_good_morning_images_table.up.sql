-- Add up migration script here
CREATE TABLE good_morning_images (
    id SERIAL PRIMARY KEY,
    image_url VARCHAR(255) NOT NULL
);
