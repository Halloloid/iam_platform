-- Add migration script here

CREATE INDEX idx_key_hash ON api_keys(key_hash);

CREATE INDEX idx_is_deleted ON api_keys(is_deleted);

CREATE INDEX idx_expires_at ON api_keys(expires_at);

CREATE INDEX idx_is_revoked ON sessions(is_revoked);

CREATE INDEX idx_expires_at_session ON sessions(expires_at);

CREATE INDEX idx_user_id ON sessions(user_id);

CREATE INDEX idx_refresh_token ON sessions(refresh_token);

CREATE INDEX idx_email ON users(email);

CREATE INDEX idx_is_deleted_user ON users(is_deleted);