use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    config::response_config::AppError,
    models::role::Role,
    repositories::{
        audit_logs::write_audit_logs,
        role::{
            all_roles, check_role_in_use, create_role, delete_role, paticular_role, role_exists,
            update_role,
        },
    },
};

pub async fn create_role_service(
    pool: &Pool<Postgres>,
    actor_id: Uuid,
    name: String,
    org_id: Uuid,
) -> Result<Uuid, AppError> {
    if role_exists(pool, org_id, &name.to_lowercase()).await? {
        return Err(AppError::Conflict(String::from("This Role Already Exists")));
    }

    let id = create_role(pool, org_id, name.clone()).await?;

    let _ = write_audit_logs(
        pool,
        "role:created",
        actor_id,
        &format!("organization:{}/role:{}", org_id, name),
    )
    .await;

    Ok(id)
}

pub async fn all_roles_service(pool: &Pool<Postgres>, org_id: Uuid) -> Result<Vec<Role>, AppError> {
    let roles = all_roles(pool, org_id).await?;

    Ok(roles)
}

pub async fn update_role_service(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    actor_id: Uuid,
    id: Uuid,
    name: String,
) -> Result<(), AppError> {
    let role = paticular_role(pool, org_id, id).await?;

    if let Some(role) = role {
        if role.name == "owner" {
            return Err(AppError::Forbidden);
        } else {
            update_role(pool, org_id, id, name.clone()).await?;

            let _ = write_audit_logs(
                pool,
                "role:updated",
                actor_id,
                &format!("organization:{}/role:{}", org_id, name),
            )
            .await;
        }
    } else {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn delete_role_service(
    pool: &Pool<Postgres>,
    org_id: Uuid,
    actor_id: Uuid,
    id: Uuid,
) -> Result<(), AppError> {
    let role = paticular_role(pool, org_id, id).await?;

    if let Some(role) = role {
        if role.name == "owner" {
            return Err(AppError::Forbidden);
        } else {
            if check_role_in_use(id, pool).await? {
                return Err(AppError::Conflict(String::from("Role is in Use")));
            } else {
                delete_role(pool, org_id, id).await?;

                let _ = write_audit_logs(
                    pool,
                    "role:delete",
                    actor_id,
                    &format!("organization:{}/role:{}", org_id, role.name),
                )
                .await;
            }
        }
    } else {
        return Err(AppError::NotFound);
    }

    Ok(())
}
