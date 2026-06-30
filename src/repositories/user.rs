use sqlx::{Pool, Postgres};

pub async fn create_user(pool:&Pool<Postgres>,email:String,password:String,name:String) -> Result<(),sqlx::Error>{
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