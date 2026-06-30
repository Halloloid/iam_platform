use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub is_deleted: bool,
}

#[derive(Debug,Deserialize)]
pub struct Create{
    pub email:String,
    pub password:String,
    pub name :String
}
