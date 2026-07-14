use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Membership {
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct AddMember {
    pub email: String,
}
