use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    config::{auth_config::AuthContext, response_config::AppError},
    handlers::require_user,
    models::audit_logs::AuditLogPagination,
    repositories::membership::check_membership,
    services::audit_logs::{org_logs_service, user_logs_service},
};

pub async fn user_logs_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Query(params): Query<AuditLogPagination>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = require_user(&auth)?;

    let data = user_logs_service(pool, user_id, params.cursor, params.limit, params.order).await?;

    Ok(Json(json!({
        "data" : &data.data,
        "next cursor" : &data.next_cursor,
        "order" : &data.order,
        "limit" : &data.limit
    })))
}

pub async fn org_logs_handler(
    State(pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Path(org_id): Path<uuid::Uuid>,
    Query(params): Query<AuditLogPagination>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = require_user(&auth)?;
    if !check_membership(&pool, user_id, org_id).await? {
        return Err(AppError::Forbidden);
    }

    let data = org_logs_service(pool, org_id, params.limit, params.cursor, params.order).await?;

    Ok(Json(json!({
        "data" : &data.data,
        "next cursor" : &data.next_cursor,
        "order" : &data.order,
        "limit" : &data.limit
    })))
}
