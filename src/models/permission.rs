use serde::{self, Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignPermissions {
    pub permission_ids: Vec<Uuid>,
}
