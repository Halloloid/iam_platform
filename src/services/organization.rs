use base64::{Engine, engine::general_purpose};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    config::response_config::AppError,
    models::organization::ListOrgsRes,
    repositories::organization::{all_organizations_asc, all_organizations_desc},
};

fn encode_cursor(created_at: DateTime<Utc>) -> String {
    general_purpose::STANDARD.encode(created_at.to_rfc3339())
}

fn decode_cursor(cursor: &str) -> Result<DateTime<Utc>, AppError> {
    let bytes = general_purpose::STANDARD
        .decode(cursor)
        .map_err(|_| AppError::BadRequest(String::from("")))?;

    let s = String::from_utf8(bytes).map_err(|_| AppError::BadRequest(String::from("")))?;

    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| AppError::BadRequest(String::from("")))
}

pub async fn all_org_service(
    user_id: Uuid,
    pool: &Pool<Postgres>,
    cursor: Option<String>,
    limit: Option<i64>,
    order: Option<String>,
) -> Result<ListOrgsRes, AppError> {
    let limit = limit.unwrap_or(10).min(100);

    let order = match order.as_deref() {
        Some("asc") => "asc",
        _ => "desc",
    };

    let decode_cursor = cursor.as_deref().map(decode_cursor).transpose()?;

    let data = match order {
        "asc" => all_organizations_asc(pool, user_id, decode_cursor, limit).await?,
        _ => all_organizations_desc(pool, user_id, decode_cursor, limit).await?,
    };

    let next_cursor = if data.len() == limit as usize {
        data.last().map(|r| encode_cursor(r.created_at))
    } else {
        None
    };

    Ok(ListOrgsRes {
        data,
        next_cursor,
        order: order.to_string(),
        limit,
    })
}
