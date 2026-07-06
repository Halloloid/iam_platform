use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Create {
    #[validate(email(message = "must be a valid email"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 chars"))]
    pub password: String,

    #[validate(length(min = 1, max = 50, message = "name is required"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(email(message = "must be a valid email"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 chars"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRes {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfile {
    #[validate(length(min = 1, max = 50, message = "name is required"))]
    pub name: String,
}
