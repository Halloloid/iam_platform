use sqlx::PgPool;
use uuid::Uuid;

use crate::config::response_config::AppError;

pub async fn add_member(pool: &PgPool, org_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO membership (user_id,org_id) VALUES ($1,$2)",
        user_id,
        org_id
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(())
}
