-- Add migration script here

ALTER TABLE sessions ADD COLUMN refresh_token TEXT NOT NULL;