use serde::{self, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct RolePermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}
