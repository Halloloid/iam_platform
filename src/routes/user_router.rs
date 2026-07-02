use axum::{Router, routing::post};
use sqlx::{Pool, Postgres};

use crate::handlers::user::{login, logout, refresh, register};

pub fn user_router() -> Router<Pool<Postgres>>{
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login",post(login))
        .route("/auth/refresh", post(refresh))
        .route("/auth/logout", post(logout))
}