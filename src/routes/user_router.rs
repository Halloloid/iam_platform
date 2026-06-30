use axum::{Router, routing::post};
use sqlx::{Pool, Postgres};

use crate::handlers::user::register;

pub fn user_router() -> Router<Pool<Postgres>>{
    Router::new()
        .route("/auth/register", post(register))
}