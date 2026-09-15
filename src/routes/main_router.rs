use axum::{
    Router, middleware,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;

use crate::{
    config::rate_limit_config::{auth_rate_limiter, login_rate_limiter}, handlers::health, routes::{
        api_keys_router::api_key_router, audit_logs::audit_logs_router,
        membership_router::membership_router, organization_router::organization_router,
        permission::permission_router, role::role_router, session::session_router,
        user_router::user_router,
    },
};

use crate::{
    handlers::user::{login, logout, refresh, register},
    middleware::auth_middleware::auth,
};

pub fn main_router(pool: Pool<Postgres>) -> Router {

    let login_route = Router::new()
        .route("/auth/login", post(login))
        .layer(login_rate_limiter());

    let auth_route = Router::new()
        .route("/auth/register", post(register))
        .route("/auth/refresh", post(refresh))
        .layer(auth_rate_limiter());
    
    
    
    let public_apis = Router::new()
        .route("/health", get(health::health))
        .merge(login_route)
        .merge(auth_route);
        
    let protected_apis = Router::new()
        .route("/auth/logout", post(logout))
        .merge(organization_router())
        .merge(user_router())
        .merge(role_router())
        .merge(permission_router())
        .merge(membership_router())
        .merge(api_key_router())
        .merge(session_router())
        .merge(audit_logs_router())
        .layer(middleware::from_fn_with_state(pool.clone(), auth));

    Router::new()
        .merge(public_apis)
        .merge(protected_apis)
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}
