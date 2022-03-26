-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR PRIMARY KEY,
    access_token VARCHAR,
    refresh_token VARCHAR,
    expires_in INT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
