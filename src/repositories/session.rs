use std::net::IpAddr;

use sqlx::{Pool, Postgres, types::ipnetwork::IpNetwork};
use uuid::Uuid;

use crate::{config::response_config::AppError};

pub async fn validate_sessions(sid:Uuid,pool:&Pool<Postgres>) -> Result<bool,AppError>{
    let Ok(revoked) = sqlx::query!("SELECT is_revoked FROM sessions WHERE id = $1 AND expires_at > NOW()",sid).fetch_one(pool).await else {
        return Err(AppError::Unauthorized);
    };

    Ok(revoked.is_revoked)
}

pub async fn creat_session(pool:&Pool<Postgres>,user_id:Uuid,device:String,ip:IpAddr,refresh_token:String)-> Result<(),AppError> {

    let ip: IpNetwork = ip.into();
    match sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id,device,ip,refresh_token) VALUES ($1,$2,$3,$4)",
        user_id,
        device,
        ip,
        refresh_token
    ).execute(pool).await {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::Database),
    }
}