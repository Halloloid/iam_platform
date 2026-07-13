use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::handlers::permission::{
    all_permission_handler, assign_permssion_handler, role_permission_handler,
};

pub fn permission_router() -> Router<PgPool> {
    Router::new()
        .route("/permission", get(all_permission_handler))
        .route(
            "/organization/{id}/role/{roleid}/permission",
            post(assign_permssion_handler).get(role_permission_handler),
        )
}
