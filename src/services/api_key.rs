use chrono::Utc;
use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::response_config::AppError,
    models::api_key::{ApiKeyListItem, CreateApiKeyResponse},
    repositories::{
        api_key::{fetch_api_keys, new_api_key},
        organization::check_permission,
    },
};

fn genrate_raw_key() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);

    format!("iam_{}", hex::encode(bytes))
}

fn hash_key(raw_key: &str) -> String {
    hex::encode(Sha256::digest(raw_key.as_bytes()))
}

pub async fn create_api_key_service(
    pool: &PgPool,
    user_id: Uuid,
    org_id: Uuid,
    api_name: String,
    permission_ids: Vec<Uuid>,
    api_expires_at: Option<i64>,
) -> Result<CreateApiKeyResponse, AppError> {
    if !check_permission(pool, user_id, org_id, "api_key:create").await? {
        return Err(AppError::Forbidden);
    }

    let raw_key = genrate_raw_key();
    let hash_key = hash_key(&raw_key);

    let expires_at = Utc::now() + chrono::Duration::days(api_expires_at.unwrap_or(30));

    let created = new_api_key(pool, org_id, api_name, hash_key, permission_ids, expires_at).await?;

    Ok(CreateApiKeyResponse {
        id: created.id,
        name: created.name,
        raw_key,
        expires_at: created.expires_at,
    })
}

pub async fn all_api_keys_service(
    pool: &PgPool,
    user_id: Uuid,
    org_id: Uuid,
) -> Result<Vec<ApiKeyListItem>, AppError> {
    if !check_permission(pool, user_id, org_id, "api_key:read").await? {
        return Err(AppError::Forbidden);
    }

    fetch_api_keys(pool, org_id).await
}
