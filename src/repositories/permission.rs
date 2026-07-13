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
