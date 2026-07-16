use axum::{
    Extension, Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::{auth_config::Claims, response_config::AppError},
    models::membership::AddMember,
    services::membership::{add_member_services, all_members_services, remove_member_service},
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
