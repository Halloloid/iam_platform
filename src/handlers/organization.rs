use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    config::{auth_config::Claims, response_config::AppError},
    models::organization::{CreateOrgReq, Organization},
    repositories::organization::create_organization,
    services::organization::all_org_service,
};

pub async fn create(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateOrgReq>,
) -> Result<impl IntoResponse, AppError> {
    body.validate().map_err(AppError::Validation)?;
    let user_id = claims.sub;

    let org_id = create_organization(user_id, body.name, &pool).await?;

    Ok((StatusCode::CREATED, Json(json!({"id":org_id}))))
}

pub async fn all_orgs(State(pool): State<PgPool>) -> Result<Json<Vec<Organization>>, AppError> {
    let data = all_org_service(&pool).await?;

    Ok(Json(data))
}
