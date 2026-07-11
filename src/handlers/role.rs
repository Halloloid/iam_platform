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
    models::role::RoleCreation,
    services::role::{all_roles_service, create_role_service, update_role_service},
};

pub async fn create_role_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(org_id): Path<Uuid>,
    Json(name): Json<RoleCreation>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    create_role_service(&pool, user_id, name.name, org_id).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message":"New Role Created"
        })),
    ))
}

pub async fn all_roles_handler(
    State(pool): State<PgPool>,
    Extension(_): Extension<Claims>,
    Path(org_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let roles = all_roles_service(&pool, org_id).await?;

    Ok((StatusCode::OK, Json(roles)))
}

pub async fn update_role_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path((org_id,role_id)): Path<(Uuid,Uuid)>,
    Json(res): Json<RoleCreation>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    let name = res.name;

    update_role_service(&pool, org_id, user_id, role_id, name).await?;

    Ok(Json(json!({
        "message":"Role name updated Successfully"
    })))
}
