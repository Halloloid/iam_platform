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
        name.to_lowercase(),
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
) -> Result<bool, AppError> {
    let exist = sqlx::query!(
        "SELECT id FROM roles WHERE name=$1 AND org_id=$2",
        name,
        org_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| AppError::Database)?;

    if exist.is_some() {
        return Ok(true);
    }

    Ok(false)
}

pub async fn all_roles(pool: &Pool<Postgres>, org_id: Uuid) -> Result<Vec<Role>, AppError> {
    let data = sqlx::query_as!(Role, "SELECT id,name FROM roles WHERE org_id = $1", org_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(data)
}

pub async fn paticular_role(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    role_id: Uuid,
) -> Result<Option<Role>, AppError> {
    let data = sqlx::query_as!(
        Role,
        "SELECT id,name FROM roles WHERE id = $1 AND org_id = $2",
        role_id,
        org_id
    )
    .fetch_optional(pool)
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
        name.to_lowercase(),
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

pub async fn check_role_in_use(role_id: Uuid, pool: &Pool<Postgres>) -> Result<bool, AppError> {
    let res = sqlx::query!(
        "SELECT COUNT(*) as count FROM member_roles WHERE role_id = $1",
        role_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(res.count.unwrap_or(0) > 0)
}

pub async fn delete_role(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    role_id: Uuid,
) -> Result<(), AppError> {
    let mut trans = pool.begin().await.map_err(|_| AppError::Database)?;

    sqlx::query!("DELETE FROM role_permissions WHERE role_id = $1", role_id)
        .execute(&mut *trans)
        .await
        .map_err(|_| AppError::Database)?;

    sqlx::query!(
        "DELETE FROM roles WHERE id =$1 and org_id= $2",
        role_id,
        org_id
    )
    .execute(&mut *trans)
    .await
    .map_err(|_| AppError::Database)?;

    trans.commit().await.map_err(|_| AppError::Database)?;

    Ok(())
}
