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
    config::{auth_config::Claims, response_config::AppError},
    models::api_key::CreateApiRequest,
    services::api_key::{all_api_keys_service, create_api_key_service, delete_api_keys},
};

pub async fn create_api_key_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(org_id): Path<Uuid>,
    Json(req): Json<CreateApiRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

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

    Ok(Json(res))
}

pub async fn all_api_keys_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(org_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    let keys = all_api_keys_service(&pool, user_id, org_id).await?;

    Ok(Json(json!({
    "data":keys
    })))
}

pub async fn delete_api_key_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path((org_id, key_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    delete_api_keys(user_id, &pool, key_id, org_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
