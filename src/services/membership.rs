use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::response_config::AppError, models::membership::Membership, repositories::{
        membership::{add_member, all_members, check_membership, delete_member}, organization::check_permission, role::{paticular_role, return_role}, user::{check_email, fnd_by_email},
    },
};

pub async fn add_member_services(
    pool: &PgPool,
    member_email: String,
    user_id: Uuid,
    org_id: Uuid,
) -> Result<(), AppError> {
    if !check_email(pool, member_email.clone()).await? {
        return Err(AppError::NotFound);
    }

    if !check_permission(pool, user_id, org_id, "member:add").await? {
        return Err(AppError::Forbidden);
    }

    let (member_id, _) = fnd_by_email(pool, member_email).await?;

    add_member(pool, org_id, member_id).await?;

    Ok(())
}

pub async fn remove_member_service(
    pool: &PgPool,
    user_id: Uuid,
    member_id: Uuid,
    org_id: Uuid,
) -> Result<(), AppError> {
    if !check_permission(pool, user_id, org_id, "member:remove").await? || user_id == member_id {
        return Err(AppError::Forbidden);
    }

    if "Owner" == return_role(pool, org_id, member_id).await? {
        return Err(AppError::Forbidden);
    }

    delete_member(pool, org_id, member_id).await?;
    Ok(())
}

pub async fn all_members_services(
    pool: &PgPool,
    org_id: Uuid,
) -> Result<Vec<Membership>, AppError> {
    let data = all_members(pool, org_id).await?;

    Ok(data)
}

pub async fn return_member_role_service(
    pool: &PgPool,
    member_id: Uuid,
    org_id: Uuid,
) -> Result<String, AppError> {
    let role = return_role(pool, org_id, member_id).await?;

    Ok(role)
}

pub async fn assign_role_service(
    pool: &PgPool,
    org_id: Uuid,
    user_id: Uuid,
    member_id: Uuid,
    role_id : Uuid
) -> Result<(),AppError> {

    if !check_permission(pool, user_id, org_id, "role:assign").await? {
        return Err(AppError::Forbidden);
    }

    if !check_membership(pool, member_id, org_id).await? {
        return Err(AppError::NotFound);
    }

    if paticular_role(pool, org_id, role_id).await?.is_none(){
        return Err(AppError::NotFound);
    }
    

    Ok(())
}