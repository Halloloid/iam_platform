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
    models::permission::AssignPermissions,
    services::permission::{
        assign_permissions_service, permission_services, role_permission_service,
    },
};

pub async fn all_permission_handler(
    State(pool): State<PgPool>,
    Extension(_): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let data = permission_services(&pool).await?;

    Ok(Json(json!({
        "data":data
    })))
}

pub async fn assign_permssion_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path((org_id, role_id)): Path<(Uuid, Uuid)>,
    Json(permission_ids): Json<AssignPermissions>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    assign_permissions_service(
        &pool,
        permission_ids.permission_ids,
        role_id,
        user_id,
        org_id,
    )
    .await?;

    Ok(Json(json!({
        "message":"Assinged All The Permissions"
    })))
}

pub async fn role_permission_handler(
    State(pool): State<PgPool>,
    Extension(_): Extension<Claims>,
    Path((org_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let permissions = role_permission_service(&pool, role_id, org_id).await?;

    Ok(Json(permissions))
}
