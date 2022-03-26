-- Add migration script here
ALTER TABLE users ALTER COLUMN expires_at TYPE TIMESTAMPTZ USING to_timestamp(expires_at);