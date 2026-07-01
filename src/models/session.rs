use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{self, Serialize};
use sqlx::{prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device: String,
    pub ip: IpAddr,
    pub refresh_token : String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}
