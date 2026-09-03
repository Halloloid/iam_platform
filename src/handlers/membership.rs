use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::{auth_config::AuthContext, response_config::AppError},
    handlers::resolver_actor,
    models::{membership::AddMember, role::RoleId},
    services::membership::{
        add_member_services, all_members_services, assign_role_service, disassign_role_service,
        remove_member_service, return_member_role_service,
    },
};

pub async fn add_member_handler(
    State(pool): State<PgPool>,
    Path(org_id): Path<Uuid>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<AddMember>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("member:add"), org_id, &pool).await?;

    add_member_services(&pool, req.email, actor.actor_id, org_id).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message":"Added new Member to The Organization"
        })),
    ))
}

pub async fn all_members_handler(
    State(pool): State<PgPool>,
    Path(org_id): Path<Uuid>,
    Extension(auth): Extension<AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, None, org_id, &pool).await?;

    let data = all_members_services(&pool, org_id, &actor).await?;

    Ok(Json(json!({
        "data":data
    })))
}

pub async fn remove_member_handler(
    State(pool): State<PgPool>,
    Path((org_id, member_id)): Path<(Uuid, Uuid)>,
    Extension(auth): Extension<AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("member:remove"), org_id, &pool).await?;

    remove_member_service(&pool, &actor, member_id, org_id).await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json("Member has Removed From the Organization"),
    ))
}

pub async fn return_role_of_member_handler(
    State(pool): State<PgPool>,
    Extension(_): Extension<AuthContext>,
    Path((org_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let role = return_member_role_service(&pool, member_id, org_id).await?;

    Ok(Json(json!({
        "role":role
    })))
}

pub async fn assign_role_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path((org_id, member_id)): Path<(Uuid, Uuid)>,
    Json(role): Json<RoleId>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("role:assign"), org_id, &pool).await?;

    assign_role_service(&pool, org_id, actor.actor_id, member_id, role.id).await?;

    Ok((StatusCode::CREATED, Json("Role has Assigned")))
}

pub async fn disassign_role_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path((org_id, member_id, role_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("role:assign"), org_id, &pool).await?;

    disassign_role_service(&pool, org_id, actor.actor_id, member_id, role_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
