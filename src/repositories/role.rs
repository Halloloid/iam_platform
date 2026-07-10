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

pub async fn role_exists(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    name:&str,
) -> Result<bool,AppError>{

    let exist = sqlx::query!(
        "SELECT id FROM roles WHERE name=$1 AND org_id=$2",
        name,
        org_id
    ).fetch_optional(pool).await.map_err(|_| AppError::Database)?;

    if let Some(_) = exist{
        return Ok(true);
    }

    Ok(false)
}