use axum::{Extension, Json, extract::State, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    config::{auth_config::Claims, response_config::AppError},
    services::permission::permission_services,
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
