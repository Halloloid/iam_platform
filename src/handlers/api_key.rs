use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::{auth_config::AuthContext, response_config::AppError},
    handlers::{require_user, resolver_actor},
    models::api_key::CreateApiRequest,
    services::api_key::{all_api_keys_service, create_api_key_service, delete_api_keys},
};

pub async fn create_api_key_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path(org_id): Path<Uuid>,
    Json(req): Json<CreateApiRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = require_user(&auth)?;

    req.validate().map_err(AppError::Validation)?;

    let res = create_api_key_service(
        &pool,
        user_id,
        org_id,
        req.name,
        req.permission_ids,
        req.expires_in_dayes,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

pub async fn all_api_keys_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path(org_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let actor = resolver_actor(&auth, Some("api_key:read"), org_id, &pool).await?;

    let keys = all_api_keys_service(&pool, actor.actor_id, org_id).await?;

    Ok(Json(json!({
    "data":keys
    })))
}

pub async fn delete_api_key_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path((org_id, key_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = require_user(&auth)?;

    delete_api_keys(user_id, &pool, key_id, org_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
