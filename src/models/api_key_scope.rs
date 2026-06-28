use serde::{self, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ApiKeyScope {
    pub api_key_id: Uuid,
    pub permission_id: Uuid,
}
