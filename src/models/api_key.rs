use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct CreatedApiKey {
    pub id: Uuid,
    pub name: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct CreateApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub raw_key: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreateApiRequest {
    #[validate(length(min = 5, message = "Name Can't be Empty"))]
    pub name: String,

    #[validate(length(min = 1, message = "There Must be atleast One Scope"))]
    pub permission_ids: Vec<Uuid>,

    pub expires_in_dayes: Option<i64>,
}

#[derive(Serialize)]
pub struct ApiKeyListItem {
    pub id: Uuid,
    pub name: String,
    pub expires_at: DateTime<Utc>,
    pub is_deleted: bool,
    pub scopes: Vec<String>,
}
