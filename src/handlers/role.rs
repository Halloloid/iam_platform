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
    models::role::RoleCreation,
    services::role::{
        all_roles_service, create_role_service, delete_role_service, update_role_service,
    },
};

pub async fn create_role_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path(org_id): Path<Uuid>,
    Json(name): Json<RoleCreation>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("role:create"), org_id, &pool).await?;

    let id = create_role_service(&pool, actor.actor_id, name.name, org_id).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message":"New Role Created",
            "id":id
        })),
    ))
}

pub async fn all_roles_handler(
    State(pool): State<PgPool>,
    Extension(_): Extension<AuthContext>,
    Path(org_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let roles = all_roles_service(&pool, org_id).await?;

    Ok((StatusCode::OK, Json(roles)))
}

pub async fn update_role_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path((org_id, role_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<RoleCreation>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("role:update"), org_id, &pool).await?;

    let name = req.name;

    update_role_service(&pool, org_id, actor.actor_id, role_id, name).await?;

    Ok(Json(json!({
        "message":"Role name updated Successfully"
    })))
}

pub async fn delete_role_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path((org_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("role:delete"), org_id, &pool).await?;

    delete_role_service(&pool, org_id, actor.actor_id, role_id).await?;

    Ok(Json(json!({
        "message" : "Role has been Deleted"
    })))
}
