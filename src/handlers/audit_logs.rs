use axum::{
    Extension, Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    config::{auth_config::Claims, response_config::AppError},
    models::audit_logs::AuditLogPagination,
    services::audit_logs::user_logs_service,
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
