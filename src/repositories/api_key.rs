use sqlx::{Pool, Postgres};

use crate::config::{auth_config::ApiKeyRecord, response_config::AppError};
use sha2::{Digest, Sha256};

pub async fn validate_api_key(
    raw_key: &str,
    pool: &Pool<Postgres>,
) -> Result<ApiKeyRecord, AppError> {
    let hash = hex::encode(Sha256::digest(raw_key.as_bytes()));

    let Ok(key) = sqlx::query!("SELECT id,org_id FROM api_keys WHERE key_hash = $1 AND is_deleted = false AND expires_at > NOW()",hash).fetch_one(pool).await else {
        return Err(AppError::Unauthorized);
    };

    let scopes= sqlx::query!("SELECT p.name FROM permissions p INNER JOIN api_keys_scopes a ON a.permission_id = p.id WHERE a.api_key_id = $1",key.id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.name)
        .collect();

    Ok(ApiKeyRecord{
        id: key.id,
        org_id: key.org_id,
        scopes,
    })
}
