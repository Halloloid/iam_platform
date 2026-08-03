use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::response_config::AppError,
    models::audit_logs::ListAuditLogs,
    repositories::audit_logs::{specific_logs_asc, specific_logs_desc},
    services::service_helper::{decode_cursor, encode_cursor},
};

pub async fn user_logs_service(
    pool: PgPool,
    user_id: Uuid,
    cursor: Option<String>,
    limit: Option<i64>,
    order: Option<String>,
) -> Result<ListAuditLogs, AppError> {
    let limit = limit.unwrap_or(10).min(100);

    let order = match order.as_deref() {
        Some("asc") => "asc",
        _ => "desc",
    };

    let decode_cursor = cursor.as_deref().map(decode_cursor).transpose()?;

    let data = match order {
        "asc" => specific_logs_asc(&pool, user_id, decode_cursor, limit).await?,
        _ => specific_logs_desc(&pool, user_id, decode_cursor, limit).await?,
    };

    let next_cursor = if data.len() == limit as usize {
        data.last().map(|x| encode_cursor(x.timestamp))
    } else {
        None
    };

    Ok(ListAuditLogs {
        data,
        next_cursor,
        order: order.to_string(),
        limit,
    })
}
