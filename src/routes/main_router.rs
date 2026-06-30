use axum::{Router,routing::get};
use sqlx::{Pool, Postgres};

use crate::{handlers::health, routes::user_router::user_router};

pub fn main_router(pool :Pool<Postgres>) -> Router{
    Router::new()
        .route("/health", get(health::health))
        .merge(user_router())
        .with_state(pool)
}