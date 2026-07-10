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
    models::role::Role,
    services::role::create_role_service,
};

pub async fn create_role_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(org_id): Path<Uuid>,
    Json(name): Json<Role>,
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
