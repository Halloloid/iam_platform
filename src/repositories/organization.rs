use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::config::response_config::AppError;

pub async fn create_organization(
    user_id: Uuid,
    name: String,
    pool: &Pool<Postgres>,
) -> Result<Uuid, AppError> {
    let mut transaction = pool.begin().await.map_err(|_| AppError::Database)?;

    let org = sqlx::query!(
        "INSERT INTO organizations (name,created_by) VALUES ($1,$2) RETURNING id",
        name,
        user_id
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    sqlx::query!(
        "INSERT INTO membership (user_id,org_id) VALUES ($1,$2)",
        user_id,
        org.id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    let role = sqlx::query!(
        "INSERT INTO roles (name,org_id) VALUES ('Owner',$1) RETURNING id",
        org.id
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    sqlx::query!(
        "INSERT INTO role_permissions (role_id,permission_id)
         SELECT $1,id FROM permissions",
        role.id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    sqlx::query!(
        "INSERT INTO member_roles (user_id,org_id,role_id) VALUES ($1,$2,$3)",
        user_id,
        org.id,
        role.id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    transaction.commit().await.map_err(|_| AppError::Database)?;

    Ok(org.id)
}
