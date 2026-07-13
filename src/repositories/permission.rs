use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::response_config::AppError, models::permission::Permission};

pub async fn all_permissions(pool: &PgPool) -> Result<Vec<Permission>, AppError> {
    let data = sqlx::query_as!(Permission, "SELECT id,name FROM permissions")
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(data)
}

pub async fn assign_permission(
    pool: &PgPool,
    permission_ids: Vec<Uuid>,
    role_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO role_permissions (role_id,permission_id)
        SELECT $1,UNNEST($2::uuid[])
        ON CONFLICT DO NOTHING",
        role_id,
        &permission_ids as &[Uuid]
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(())
}

pub async fn role_permission(
    pool: &PgPool,
    role_id: Uuid,
    org_id: Uuid,
) -> Result<Vec<Permission>, AppError> {
    let data = sqlx::query_as!(
        Permission,
        "SELECT p.id,p.name FROM permissions p 
        INNER JOIN role_permissions rp ON rp.permission_id = p.id
        INNER JOIN roles r ON r.id = rp.role_id
        WHERE rp.role_id = $1 AND
        r.org_id = $2",
        role_id,
        org_id
    )
    .fetch_all(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(data)
}
