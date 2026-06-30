-- Add migration script here
ALTER TABLE organizations ADD COLUMN created_by UUID NOT NULL REFERENCES users(id);