use sqlx::{Pool, Postgres};

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