use std::net::IpAddr;

use sqlx::{Pool, Postgres, types::ipnetwork::IpNetwork};
use uuid::Uuid;

use crate::config::response_config::AppError;

pub async fn validate_sessions(sid: Uuid, pool: &Pool<Postgres>) -> Result<bool, AppError> {
    let Ok(revoked) = sqlx::query!(
        "SELECT is_revoked FROM sessions WHERE id = $1 AND expires_at > NOW()",
        sid
    )
    .fetch_one(pool)
    .await
    else {
        return Err(AppError::Unauthorized);
    };

    Ok(revoked.is_revoked)
}

pub async fn creat_session(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    device: String,
    ip: IpAddr,
    refresh_token: String,
) -> Result<(), AppError> {
    let ip: IpNetwork = ip.into();
    match sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id,device,ip,refresh_token) VALUES ($1,$2,$3,$4)",
        user_id,
        device,
        ip,
        refresh_token
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::Database),
    }
}

pub async fn find_active_session(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    device: &str,
) -> Result<Option<Uuid>, AppError> {
    let session = sqlx::query!(
        "SELECT id FROM sessions WHERE user_id = $1 AND device = $2 AND is_revoked = false",
        user_id,
        device
    )
    .fetch_optional(pool)
    .await?;

    if let Some(session) = session {
        return Ok(Some(session.id));
    }

    Ok(None)
}

pub async fn update_session(
    pool: &Pool<Postgres>,
    refresh_token: &str,
    ip: IpAddr,
    id: Uuid,
) -> Result<(), AppError> {
    let ip: IpNetwork = ip.into();
    sqlx::query!(
        "UPDATE sessions SET refresh_token=$1,ip=$2,expires_at = NOW() + INTERVAL '7 days' WHERE id = $3",
        refresh_token,
        ip,
        id
    ).execute(pool)
    .await?;

    Ok(())
}

pub async fn fnd_by_refresh_token(
    pool: &Pool<Postgres>,
    refresh_token: String,
) -> Result<(Uuid,Uuid), AppError> {

    let rec = sqlx::query!("SELECT id,user_id FROM sessions WHERE refresh_token = $1",refresh_token).fetch_optional(pool).await?;

    if let Some(rec) = rec {
        return Ok((rec.id,rec.user_id));
    } 
    Err(AppError::Unauthorized)   
}
