use axum::{middleware, Router, routing::{get,post}};
use sqlx::{Pool, Postgres};

use crate::{handlers::health, routes::user_router::user_router};
use crate::{
    handlers::user::{login, logout, refresh, register},
    middleware::auth_middleware::auth,
};


pub fn main_router(pool: Pool<Postgres>) -> Router {
    let public_apis = Router::new()
        .route("/health", get(health::health))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .route("/auth/logout", post(logout));

    let protected_apis = Router::new()
        .merge(user_router())
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth));

    Router::new()
        .merge(public_apis)
        .merge(protected_apis)
        .with_state(pool)
}
