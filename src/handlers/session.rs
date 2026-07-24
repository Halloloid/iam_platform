use axum::{
    Extension, Json,
    extract::{Path, State},
    http::{HeaderMap, header},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::{auth_config::Claims, response_config::AppError},
    services::session::{list_sessions, revoke_session_service},
};

pub async fn list_session_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    let device = headers
        .get(header::USER_AGENT)
        .and_then(|x| x.to_str().ok())
        .unwrap_or("Unknown Device")
        .to_string();

    let data = list_sessions(&pool, user_id, device).await?;

    Ok(Json(json!({
        "data":data
    })))
}

pub async fn revoke_session_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(session_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

    revoke_session_service(&pool, user_id, session_id).await?;

    Ok(Json(json!({
        "message":"Session Revoked"
    })))
}
