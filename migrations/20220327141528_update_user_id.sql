-- Add migration script here
ALTER TABLE users ALTER COLUMN id TYPE UUID USING gen_random_uuid();