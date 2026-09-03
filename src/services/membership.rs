use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::{auth_config::AuthActor, response_config::AppError},
    models::membership::Membership,
    repositories::{
        audit_logs::write_audit_logs,
        membership::{
            add_member, all_members, assign_role, check_membership, delete_member, disassign_role,
        },
        role::{paticular_role, return_role},
        user::{check_email, fnd_by_email},
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

    let (member_id, _) = fnd_by_email(pool, member_email).await?;

    if check_membership(pool, member_id, org_id).await? {
        return Err(AppError::Conflict(format!("Member Already Exist")));
    }

    add_member(pool, org_id, member_id).await?;

    let _ = write_audit_logs(
        pool,
        "member:added",
        user_id,
        &format!("organization:{}/member:{}", org_id, member_id),
    )
    .await;

    Ok(())
}

pub async fn remove_member_service(
    pool: &PgPool,
    actor: &AuthActor,
    member_id: Uuid,
    org_id: Uuid,
) -> Result<(), AppError> {
    if let Some(user_id) = actor.user_id
        && user_id == member_id
    {
        return Err(AppError::Forbidden);
    }

    if "owner" == return_role(pool, org_id, member_id).await? {
        return Err(AppError::Forbidden);
    }

    delete_member(pool, org_id, member_id).await?;

    let _ = write_audit_logs(
        pool,
        "member:removed",
        actor.actor_id,
        &format!("organization:{}/member:{}", org_id, member_id),
    )
    .await;

    Ok(())
}

pub async fn all_members_services(
    pool: &PgPool,
    org_id: Uuid,
    actor: &AuthActor,
) -> Result<Vec<Membership>, AppError> {
    if let Some(member_id) = actor.user_id
        && !check_membership(pool, member_id, org_id).await?
    {
        return Err(AppError::Forbidden);
    }

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
    _actor_id: Uuid,
    member_id: Uuid,
    role_id: Uuid,
) -> Result<(), AppError> {
    if !check_membership(pool, member_id, org_id).await? {
        return Err(AppError::NotFound);
    }

    if paticular_role(pool, org_id, role_id).await?.is_none() {
        return Err(AppError::NotFound);
    }

    assign_role(pool, role_id, member_id, org_id).await?;

    Ok(())
}

pub async fn disassign_role_service(
    pool: &PgPool,
    org_id: Uuid,
    _actor_id: Uuid,
    member_id: Uuid,
    role_id: Uuid,
) -> Result<(), AppError> {
    if !check_membership(pool, member_id, org_id).await? {
        return Err(AppError::NotFound);
    }

    if paticular_role(pool, org_id, role_id).await?.is_none() {
        return Err(AppError::NotFound);
    }

    if let Some(r) = paticular_role(pool, org_id, role_id).await?
        && r.name == "owner"
    {
        return Err(AppError::Conflict(format!(
            "Owner trying to Remove Owner Role"
        )));
    };

    disassign_role(pool, role_id, member_id, org_id).await?;

    Ok(())
}
