use sqlx::PgPool;
use uuid::Uuid;

use crate::config::response_config::AppError;

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
