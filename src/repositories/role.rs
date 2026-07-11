use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{config::response_config::AppError, models::role::Role};

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
    name: &str,
) -> Result<Option<Uuid>, AppError> {
    let exist = sqlx::query!(
        "SELECT id FROM roles WHERE name=$1 AND org_id=$2",
        name,
        org_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| AppError::Database)?;

    if let Some(rec) = exist {
        return Ok(Some(rec.id));
    }

    Ok(None)
}

pub async fn all_roles(pool: &Pool<Postgres>, org_id: Uuid) -> Result<Vec<Role>, AppError> {
    let data = sqlx::query_as!(Role, "SELECT id,name FROM roles WHERE org_id = $1", org_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(data)
}

pub async fn update_role(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    id: Uuid,
    name: String,
) -> Result<(), AppError> {
    let row = sqlx::query!(
        "UPDATE roles SET name = $1 WHERE id = $2 and org_id = $3",
        name,
        id,
        org_id
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::Database)?;

    if row.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}
