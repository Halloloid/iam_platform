-- Add migration script here

CREATE TABLE IF NOT EXISTS users(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(225) NOT NULL UNIQUE CHECK(email LIKE '%@%'),
    password VARCHAR(225) NOT NULL,
    name VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_deleted BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS organizations(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_deleted BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS membership(
    user_id UUID NOT NULL REFERENCES users(id),
    org_id UUID NOT NULL REFERENCES organizations(id),
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id,org_id)
);

CREATE TABLE IF NOT EXISTS roles(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    org_id UUID NOT NULL REFERENCES organizations(id),
    UNIQUE(name,org_id)
);

CREATE TABLE IF NOT EXISTS permissions(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS role_permissions(
    role_id UUID NOT NULL REFERENCES roles(id),
    permission_id UUID NOT NULL REFERENCES permissions(id),
    PRIMARY KEY(role_id,permission_id)
);

CREATE TABLE IF NOT EXISTS member_roles(
    user_id UUID NOT NULL,
    org_id UUID NOT NULL,
    role_id UUID NOT NULL REFERENCES roles(id),
    FOREIGN KEY (user_id,org_id) REFERENCES membership(user_id,org_id),
    PRIMARY KEY (user_id,org_id,role_id)
);

CREATE TABLE IF NOT EXISTS sessions(
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    device VARCHAR(30) NOT NULL,
    ip INET NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '7 days',
    is_revoked BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS api_keys(
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id),
    name VARCHAR(30) NOT NULL,
    key_hash VARCHAR(100) UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '1 month',
    is_deleted BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS api_keys_scopes(
    api_key_id UUID NOT NULL REFERENCES api_keys(id),
    permission_id UUID NOT NULL REFERENCES permissions(id),
    PRIMARY KEY(api_key_id,permission_id)
);

CREATE TABLE IF NOT EXISTS audit_logs(
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID NOT NULL,
    action TEXT NOT NULL,
    resource TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW() 
);


