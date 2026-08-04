use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::response_config::AppError, models::audit_logs::AuditLogs};

pub async fn write_audit_logs(
    pool: &PgPool,
    action: &str,
    actor_id: Uuid,
    resourse: &str,
) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO audit_logs (actor_id,action,resource) VALUES ($1,$2,$3)",
        actor_id,
        action,
        resourse
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(())
}

pub async fn specific_logs_asc(
    pool: &PgPool,
    actor_id: Uuid,
    cursor: Option<DateTime<Utc>>,
    limit: i64,
) -> Result<Vec<AuditLogs>, AppError> {
    let data = sqlx::query_as!(
        AuditLogs,
        "SELECT id,actor_id,action,resource,timestamp FROM audit_logs WHERE
        actor_id = $1
        AND ($2::timestamptz IS NULL OR timestamp > $2)
        ORDER BY timestamp ASC
        LIMIT $3",
        actor_id,
        cursor,
        limit
    )
    .fetch_all(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(data)
}

pub async fn specific_logs_desc(
    pool: &PgPool,
    actor_id: Uuid,
    cursor: Option<DateTime<Utc>>,
    limit: i64,
) -> Result<Vec<AuditLogs>, AppError> {
    let data = sqlx::query_as!(
        AuditLogs,
        "SELECT id,actor_id,action,resource,timestamp FROM audit_logs WHERE
        actor_id = $1
        AND ($2::timestamptz IS NULL OR timestamp < $2)
        ORDER BY timestamp DESC
        LIMIT $3",
        actor_id,
        cursor,
        limit
    )
    .fetch_all(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(data)
}

pub async fn org_specificlog_asc(
    pool: &PgPool,
    org_id: Uuid,
    cursor: Option<DateTime<Utc>>,
    limit: i64,
) -> Result<Vec<AuditLogs>, AppError> {
    let pattern = format!("organization:{}%", org_id);

    let data = sqlx::query_as!(
        AuditLogs,
        "SELECT id,actor_id,action,resource,timestamp FROM audit_logs WHERE resource LIKE $1
        AND ($2::timestamptz IS NULL OR timestamp > $2)
        ORDER BY timestamp ASC
        LIMIT $3",
        pattern,
        cursor,
        limit
    )
    .fetch_all(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(data)
}

pub async fn org_specificlog_desc(
    pool: &PgPool,
    org_id: Uuid,
    cursor: Option<DateTime<Utc>>,
    limit: i64,
) -> Result<Vec<AuditLogs>, AppError> {
    let pattern = format!("organization:{}%", org_id);

    let data = sqlx::query_as!(
        AuditLogs,
        "SELECT id,actor_id,action,resource,timestamp FROM audit_logs WHERE resource LIKE $1
        AND ($2::timestamptz IS NULL OR timestamp < $2)
        ORDER BY timestamp DESC
        LIMIT $3",
        pattern,
        cursor,
        limit
    )
    .fetch_all(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(data)
}
