use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::response_config::AppError,
    models::permission::Permission,
    repositories::{
        organization::check_permission,
        permission::{all_permissions, assign_permission},
        role::paticular_role,
    },
};

pub async fn permission_services(pool: &PgPool) -> Result<Vec<Permission>, AppError> {
    let data = all_permissions(pool).await?;

    Ok(data)
}

pub async fn assign_permissions_service(
    pool: &PgPool,
    permission_ids: Vec<Uuid>,
    role_id: Uuid,
    user_id: Uuid,
    org_id: Uuid,
) -> Result<(), AppError> {
    if permission_ids.is_empty() {
        return Err(AppError::BadRequest("No Permission Provided".into()));
    }

    let allowed = check_permission(pool, user_id, org_id, "permission:assign").await?;

    if !allowed {
        return Err(AppError::Forbidden);
    }

    if paticular_role(pool, org_id, role_id).await?.is_none() {
        return Err(AppError::NotFound);
    }

    assign_permission(pool, permission_ids, role_id).await?;

    Ok(())
}
