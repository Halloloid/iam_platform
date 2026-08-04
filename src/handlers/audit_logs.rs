use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    config::{auth_config::Claims, response_config::AppError},
    models::audit_logs::AuditLogPagination,
    services::audit_logs::{org_logs_service, user_logs_service},
};

pub async fn user_logs_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<AuditLogPagination>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.sub;

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
    Extension(claims): Extension<Claims>,
    Path(org_id): Path<uuid::Uuid>,
    Query(params): Query<AuditLogPagination>,
) -> Result<impl IntoResponse, AppError> {
    let _ = claims.sub;

    let data = org_logs_service(pool, org_id, params.limit, params.cursor, params.order).await?;

    Ok(Json(json!({
        "data" : &data.data,
        "next cursor" : &data.next_cursor,
        "order" : &data.order,
        "limit" : &data.limit
    })))
}
