use std::net::IpAddr;

use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::{Pool, Postgres};

use crate::{
    config::{
        auth_config::{create_token, hash_password, verify_password},
        response_config::AppError,
    },
    models::user::{Create, LoginReq, LoginRes},
    repositories::{
        self,
        session::{creat_session, find_active_session, fnd_by_refresh_token, update_session},
        user::{check_email, fnd_by_email},
    },
};

pub async fn register(pool: &Pool<Postgres>, req: Create) -> Result<(), AppError> {
    if !check_email(pool, req.email.to_string()).await? {
        let hashed_password = hash_password(&req.password)?;
        repositories::user::create_user(pool, req.email, hashed_password, req.name).await?;
    } else {
        return Err(AppError::Conflict(String::from("Already Registerd")));
    }

    Ok(())
}

pub async fn login(
    pool: &Pool<Postgres>,
    req: LoginReq,
    ip: IpAddr,
    device: String,
) -> Result<LoginRes, AppError> {
    let (user_id, pswd) = fnd_by_email(pool, req.email).await?;

    if verify_password(&req.password, &pswd)? {
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);

        let refresh_token = hex::encode(Sha256::digest(bytes));

        let session_id = find_active_session(pool, user_id, &device).await?;

        match session_id {
            Some(id) => {
                update_session(pool, &refresh_token, ip, id).await?;
            }
            None => {
                creat_session(pool, user_id, device, ip, refresh_token.clone()).await?;
            }
        }

        let access_token = create_token(user_id)?;

        return Ok(LoginRes {
            access_token,
            refresh_token,
            expires_in: 900,
        });
    }
    Err(AppError::Unauthorized)
}

pub async fn refresh(pool: &Pool<Postgres>, req: String, ip: IpAddr) -> Result<LoginRes, AppError> {
    let (sid, user_id) = fnd_by_refresh_token(pool, req).await?;

    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);

    let refresh_token = hex::encode(Sha256::digest(bytes));

    update_session(pool, &refresh_token, ip, sid).await?;

    let access_token = create_token(user_id)?;

    Ok(LoginRes {
        access_token,
        refresh_token,
        expires_in: 900,
    })
}
