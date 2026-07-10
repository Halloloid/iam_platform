use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    config::response_config::AppError, models::role::Role, repositories::{organization::check_permission, role::{all_roles, create_role, role_exists}},
};

pub async fn create_role_service(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    name: String,
    org_id: Uuid,
) -> Result<(), AppError> {
    let allowed = check_permission(pool, user_id, org_id, "role:create").await?;

    if !allowed {
        return Err(AppError::Forbidden);
    }

    if role_exists(pool, org_id, &name).await? {
        return Err(AppError::Conflict(String::from("This Role Already Exists")));
    }
    
    create_role(pool, org_id, name).await?;

    Ok(())
}

pub async fn all_roles_service(
    pool: &Pool<Postgres>,
    org_id: Uuid
) -> Result<Vec<Role>,AppError>{
    let roles = all_roles(pool, org_id).await?;

    Ok(roles)
}
