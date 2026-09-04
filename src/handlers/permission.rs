use axum::{
    Extension, Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::{auth_config::AuthContext, response_config::AppError},
    handlers::resolver_actor,
    models::permission::AssignPermissions,
    services::permission::{
        assign_permissions_service, delete_permission_of_role_service, permission_services,
        role_permission_service,
    },
};

pub async fn all_permission_handler(
    State(pool): State<PgPool>,
    Extension(_): Extension<AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    let data = permission_services(&pool).await?;

    Ok(Json(json!({
        "data":data
    })))
}

pub async fn assign_permssion_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path((org_id, role_id)): Path<(Uuid, Uuid)>,
    Json(permission_ids): Json<AssignPermissions>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("permission:assign"), org_id, &pool).await?;

    assign_permissions_service(
        &pool,
        permission_ids.permission_ids,
        role_id,
        actor.actor_id,
        org_id,
    )
    .await?;

    Ok(Json(json!({
        "message":"Assinged All The Permissions"
    })))
}

pub async fn delete_permission_of_role_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path((org_id, role_id)): Path<(Uuid, Uuid)>,
    Json(permission_ids): Json<AssignPermissions>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("permission:assign"), org_id, &pool).await?;

    delete_permission_of_role_service(
        actor.actor_id,
        org_id,
        &pool,
        permission_ids.permission_ids,
        role_id,
    )
    .await?;

    Ok(Json(json!({
        "message":"Removed The Permissions"
    })))
}

pub async fn role_permission_handler(
    State(pool): State<PgPool>,
    Extension(_): Extension<AuthContext>,
    Path((org_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let permissions = role_permission_service(&pool, role_id, org_id).await?;

    Ok(Json(permissions))
}
