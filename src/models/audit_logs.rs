use chrono::{DateTime, Utc};
use serde::{self, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AuditLogs {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub resourse: String,
    pub timestamp: DateTime<Utc>,
}
