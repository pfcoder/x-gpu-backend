-- Add migration script here
ALTER TABLE users RENAME COLUMN expires_in TO expires_at;