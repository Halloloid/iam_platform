use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::response_config::AppError, models::membership::Membership};

pub async fn add_member(pool: &PgPool, org_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO membership (user_id,org_id) VALUES ($1,$2)",
        user_id,
        org_id
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(())
}

pub async fn all_members(pool: &PgPool, org_id: Uuid) -> Result<Vec<Membership>, AppError> {
    let data = sqlx::query_as!(
        Membership,
        "SELECT user_id,joined_at FROM membership WHERE org_id = $1",
        org_id
    )
    .fetch_all(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(data)
}

pub async fn delete_member(pool: &PgPool, org_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to create transaction:{:?}", e);
        AppError::Database
    })?;

    sqlx::query!(
        "DELETE FROM member_roles WHERE user_id = $1 AND org_id = $2",
        user_id,
        org_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete from member roles:{:?}", e);
        AppError::Database
    })?;

    let rows = sqlx::query!(
        "DELETE FROM membership WHERE user_id = $1 AND org_id = $2",
        user_id,
        org_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete from Members : {:?}", e);
        AppError::Database
    })?;

    if rows.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    tx.commit().await.map_err(|_| AppError::Database)?;

    Ok(())
}
