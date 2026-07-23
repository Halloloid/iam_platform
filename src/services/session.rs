use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::response_config::AppError,
    models::session::SessionResponse,
    repositories::session::{fetch_user_sessions, find_active_session},
};

pub async fn list_sessions(
    pool: &PgPool,
    user_id: Uuid,
    device: String,
) -> Result<Vec<SessionResponse>, AppError> {
    let current_session_id = find_active_session(pool, user_id, &device).await?;

    let Some(current_id) = current_session_id else {
        return Err(AppError::Unauthorized);
    };

    let data = fetch_user_sessions(user_id, pool).await?;

    Ok(data
        .iter()
        .map(|x| SessionResponse {
            id: x.id,
            device: device.clone(),
            ip: x.ip.to_string(),
            created_at: x.created_at,
            expires_at: x.expires_at,
            is_current: x.id == current_id,
            is_revoked: x.is_revoked,
        })
        .collect())
}
