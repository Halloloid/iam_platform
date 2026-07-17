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
    config::{auth_config::Claims, response_config::AppError},
    models::{membership::AddMember, role::RoleId},
    services::membership::{
        add_member_services, all_members_services, assign_role_service, remove_member_service,
        return_member_role_service,
    },
};

pub async fn add_member_handler(
    State(pool): State<PgPool>,
    Path(org_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<AddMember>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    add_member_services(&pool, req.email, user_id, org_id).await?;

    Ok(Json(json!({
        "message":"Added new Member to The Organization"
    })))
}

pub async fn all_members_handler(
    State(pool): State<PgPool>,
    Path(org_id): Path<Uuid>,
    Extension(_): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let data = all_members_services(&pool, org_id).await?;

    Ok(Json(json!({
        "data":data
    })))
}

pub async fn remove_member_handler(
    State(pool): State<PgPool>,
    Path((org_id, member_id)): Path<(Uuid, Uuid)>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    remove_member_service(&pool, user_id, member_id, org_id).await?;

    Ok(Json("Member has Removed From the Organization"))
}

pub async fn return_role_of_member_handler(
    State(pool): State<PgPool>,
    Extension(_): Extension<Claims>,
    Path((org_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let role = return_member_role_service(&pool, member_id, org_id).await?;

    Ok(Json(json!({
        "role":role
    })))
}

pub async fn assign_role_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path((org_id, member_id)): Path<(Uuid, Uuid)>,
    Json(role): Json<RoleId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    assign_role_service(&pool, org_id, user_id, member_id, role.id).await?;

    Ok((StatusCode::CREATED, Json("Role has Assigned")))
}
