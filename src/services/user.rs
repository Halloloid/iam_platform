use sqlx::{Pool, Postgres};

use crate::{config::response_config::AppError, models::user::Create, repositories};

pub async fn register(pool:&Pool<Postgres>,req:Create) -> Result<(),AppError>{
    repositories::user::create_user(pool, req.email, req.password, req.name).await?;

    Ok(())
}