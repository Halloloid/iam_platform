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
    models::membership::AddMember,
    services::membership::add_member_services,
};

pub async fn add_member_handler(
    State(pool): State<PgPool>,
    Path(org_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<AddMember>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    add_member_services(&pool, req.email, user_id, org_id).await?;

    Ok(Json(json!({
        "message":"Added new Member to The Organization"
    })))
}
