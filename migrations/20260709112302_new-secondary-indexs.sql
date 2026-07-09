-- Add migration script here

CREATE INDEX idx_member_user_id ON membership(user_id);

CREATE INDEX idx_member_org_id ON membership(org_id);

CREATE INDEX idx_org_is_delted ON organizations(is_deleted);

CREATE INDEX idx_org_created_at ON organizations(created_at);

CREATE INDEX idx_role_permission_role_id ON role_permissions(role_id);

CREATE INDEX idx_role_permission_permission_id ON role_permissions(permission_id);

CREATE INDEX idx_member_role_user_id ON member_roles(user_id);

CREATE INDEX idx_member_role_org_id ON member_roles(org_id);

CREATE INDEX idx_member_role_role_id ON member_roles(role_id);

CREATE INDEX idx_permission_name ON permissions(name);