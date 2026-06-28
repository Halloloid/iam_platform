use serde::{self, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub org_id: Uuid,
}
