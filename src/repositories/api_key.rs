use chrono::{DateTime, Utc};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::{
    config::{auth_config::ApiKeyRecord, response_config::AppError},
    models::api_key::CreatedApiKey,
};
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

    Ok(ApiKeyRecord {
        id: key.id,
        org_id: key.org_id,
        scopes,
    })
}

pub async fn new_api_key(
    pool: &PgPool,
    org_id: Uuid,
    name: String,
    key_hash: String,
    permission_ids: Vec<Uuid>,
    expires_at: DateTime<Utc>,
) -> Result<CreatedApiKey, AppError> {
    let mut tx = pool.begin().await.map_err(|_| AppError::Database)?;

    let key = sqlx::query_as!(
        CreatedApiKey,
        "INSERT INTO api_keys (org_id,name,key_hash,expires_at) VALUES ($1,$2,$3,$4) RETURNING id,name,expires_at",
        org_id,
        name,
        key_hash,
        expires_at
    ).fetch_one(&mut *tx)
    .await.map_err(|_| AppError::Database)?;

    sqlx::query!(
        "INSERT INTO api_keys_scopes (api_key_id,permission_id) 
        SELECT $1, UNNEST($2::uuid[])
        ON CONFLICT DO NOTHING",
        key.id,
        &permission_ids as &[Uuid]
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| AppError::Database)?;

    tx.commit().await.map_err(|_| AppError::Database)?;

    Ok(key)
}
