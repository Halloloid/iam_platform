use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    config::response_config::AppError,
    models::role::Role,
    repositories::{
        organization::check_permission,
        role::{
            all_roles, check_role_in_use, create_role, delete_role, paticular_role, role_exists,
            update_role,
        },
    },
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

pub async fn all_roles_service(pool: &Pool<Postgres>, org_id: Uuid) -> Result<Vec<Role>, AppError> {
    let roles = all_roles(pool, org_id).await?;

    Ok(roles)
}

pub async fn update_role_service(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    user_id: Uuid,
    id: Uuid,
    name: String,
) -> Result<(), AppError> {
    let allowed = check_permission(pool, user_id, org_id, "role:update").await?;

    if !allowed {
        return Err(AppError::Forbidden);
    }

    let role = paticular_role(pool, org_id, id).await?;

    if let Some(role) = role {
        if role.name == "Owner" {
            return Err(AppError::Forbidden);
        } else {
            update_role(pool, org_id, id, name).await?;
        }
    } else {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn delete_role_service(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    user_id: Uuid,
    id: Uuid,
) -> Result<(), AppError> {
    let allowed = check_permission(pool, user_id, org_id, "role:delete").await?;

    if !allowed {
        return Err(AppError::Forbidden);
    }

    let role = paticular_role(pool, org_id, id).await?;

    if let Some(role) = role {
        if role.name == "Owner" {
            return Err(AppError::Forbidden);
        } else {
            if check_role_in_use(id, pool).await? {
                return Err(AppError::Conflict(String::from("Role is in Use")));
            } else {
                delete_role(pool, org_id, id).await?;
            }
        }
    } else {
        return Err(AppError::NotFound);
    }

    Ok(())
}
