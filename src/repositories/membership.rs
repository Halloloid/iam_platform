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

pub async fn delete_member(
    pool: &PgPool,
    org_id: Uuid,
    user_id:Uuid
) -> Result<(),AppError> {

    let tx = pool.begin().await.map_err(|_| AppError::Database)?;
    Ok(())
}