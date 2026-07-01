use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::config::response_config::AppError;

pub async fn validate_sessions(sid:Uuid,pool:&Pool<Postgres>) -> Result<bool,AppError>{
    let Ok(revoked) = sqlx::query!("SELECT is_revoked FROM sessions WHERE id = $1 AND expires_at > NOW()",sid).fetch_one(pool).await else {
        return Err(AppError::Unauthorized);
    };

    Ok(revoked.is_revoked)
}