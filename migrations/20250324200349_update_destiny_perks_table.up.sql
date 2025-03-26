-- Add up migration script here
ALTER TABLE destiny_perks
ADD COLUMN description TEXT NOT NULL DEFAULT '';