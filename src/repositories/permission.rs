use sqlx::PgPool;

use crate::{config::response_config::AppError, models::permission::Permission};

pub async fn all_permissions(pool: &PgPool) -> Result<Vec<Permission>, AppError> {
    let data = sqlx::query_as!(Permission, "SELECT id,name FROM permissions")
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(data)
}
