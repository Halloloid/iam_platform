use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrgReq {
    #[validate(length(min = 1, max = 100, message = "name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ListOrgsRes {
    pub data: Vec<Organization>,
    pub next_cursor: Option<String>,
    pub order: String,
    pub limit: i64,
}

#[derive(Debug, Deserialize)]
pub struct OrgPaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub order: Option<String>,
}
