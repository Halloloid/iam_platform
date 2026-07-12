use sqlx::PgPool;

use crate::{
    config::response_config::AppError, models::permission::Permission,
    repositories::permission::all_permissions,
};

pub async fn permission_services(pool: &PgPool) -> Result<Vec<Permission>, AppError> {
    let data = all_permissions(pool).await?;

    Ok(data)
}
