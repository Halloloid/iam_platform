use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::config::response_config::AppError;

pub async fn create_user(pool:&Pool<Postgres>,email:String,password:String,name:String) -> Result<(),AppError>{
    sqlx::query_as!(
        Create,
        "INSERT INTO users (email,password,name) VALUES ($1,$2,$3)",
        email,
        password,
        name
    ).execute(pool)
    .await?;

    Ok(())
}

pub async fn check_email(pool:&Pool<Postgres>,email:String) -> Result<bool,AppError> {
    let exists = sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",email).fetch_one(pool).await?;

    Ok(exists.exists.unwrap())
}

pub async fn fnd_by_email(pool:&Pool<Postgres>,email:String) -> Result<(Uuid,String),AppError> {

    let paswd = sqlx::query!("SELECT id,password FROM users WHERE email = $1",email).fetch_optional(pool).await?;

    if let Some(pswd) = paswd{
        return Ok((pswd.id,pswd.password));
    }else {
        return Err(AppError::Unauthorized);
    }
}

