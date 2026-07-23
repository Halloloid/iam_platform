use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub device: String,
    pub ip: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub device: String,
    pub ip: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_current: bool,
    pub is_revoked: bool,
}

#[derive(Debug, Deserialize)]
pub struct ReqToken {
    pub refresh_token: String,
}
