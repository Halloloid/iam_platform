use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::config::response_config::AppError;

pub async fn create_role(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    name: String,
) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO roles (name,org_id) VALUES ($1,$2)",
        name,
        org_id
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(())
}
