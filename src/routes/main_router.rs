use axum::{
    Router, middleware,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;

use crate::{
    handlers::health,
    routes::{
        api_keys_router::api_key_router, membership_router::membership_router,
        organization_router::organization_router, permission::permission_router, role::role_router,
        user_router::user_router,
    },
};
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
        .merge(organization_router())
        .merge(user_router())
        .merge(role_router())
        .merge(permission_router())
        .merge(membership_router())
        .merge(api_key_router())
        .layer(middleware::from_fn_with_state(pool.clone(), auth));

    Router::new()
        .merge(public_apis)
        .merge(protected_apis)
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}
