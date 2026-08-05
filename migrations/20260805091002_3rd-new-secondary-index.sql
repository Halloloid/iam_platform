-- Add migration script here

CREATE INDEX idx_api_key_id ON api_keys_scopes(api_key_id);

CREATE INDEX idx_permission_id ON api_keys_scopes(permission_id);

CREATE INDEX idx_audit_actor_id ON audit_logs(actor_id);

CREATE INDEX idx_audit_resource ON audit_logs(resource);

CREATE INDEX idx_audit_timestamp ON audit_logs(timestamp);