use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AuditLogs {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub resource: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ListAuditLogs {
    pub data: Vec<AuditLogs>,
    pub next_cursor: Option<String>,
    pub order: String,
    pub limit: i64,
}

#[derive(Debug, Deserialize)]
pub struct AuditLogPagination {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub order: Option<String>,
}
