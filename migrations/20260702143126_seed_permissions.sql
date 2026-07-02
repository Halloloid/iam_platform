-- Add migration script here
INSERT INTO permissions (name) VALUES
    ('user:create'),
    ('user:read'),
    ('user:update'),
    ('user:delete'),
    ('organization:read'),
    ('organization:update'),
    ('role:create'),
    ('role:update'),
    ('role:delete'),
    ('permission:assign'),
    ('member:add'),
    ('member:remove'),
    ('api_key:create'),
    ('api_key:read'),
    ('api_key:delete'),
    ('audit_log:read');