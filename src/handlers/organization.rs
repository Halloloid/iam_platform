use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    config::{auth_config::Claims, response_config::AppError},
    models::organization::CreateOrgReq,
    repositories::organization::create_organization,
};

pub async fn create(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateOrgReq>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    let org_id = create_organization(user_id, body.name, &pool).await?;

    Ok((StatusCode::CREATED, Json(json!({"id":org_id}))))
}
