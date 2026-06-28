use chrono::{DateTime, Utc};
use serde::{self, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub expires_at: DateTime<Utc>,
    pub is_deleted: bool,
}
