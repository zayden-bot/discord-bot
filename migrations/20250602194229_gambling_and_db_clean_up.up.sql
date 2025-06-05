-- Add up migration script here
DROP TABLE level_roles;
DROP TABLE patreon_cache;
DROP TABLE questions;
DROP TABLE server_rules;
DROP TABLE support_faq;

ALTER TABLE gambling
RENAME COLUMN cash TO coins;

ALTER TABLE gambling
RENAME COLUMN diamonds TO gems;

ALTER TABLE gambling
RENAME COLUMN work TO stamina;

ALTER TABLE gambling
ALTER COLUMN stamina DROP DEFAULT;

ALTER TABLE gambling
ALTER COLUMN stamina TYPE INT USING 0;

ALTER TABLE gambling
ALTER COLUMN stamina SET DEFAULT 1;