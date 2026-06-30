use sqlx::{Pool, Postgres};

use crate::{config::{auth_config::hash_password, response_config::AppError}, models::user::Create, repositories::{self, user::check_email}};

pub async fn register(pool:&Pool<Postgres>,req:Create) -> Result<(),AppError>{

    if !check_email(pool, req.email.to_string()).await? {
        let hashed_password = hash_password(&req.password)?;
        repositories::user::create_user(pool, req.email, hashed_password, req.name).await?;
    }else {
        return Err(AppError::Conflict(String::from("Already Registerd")))
    }
    
    Ok(())
}