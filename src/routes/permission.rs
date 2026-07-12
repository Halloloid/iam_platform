use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::permission::all_permission_handler;

pub fn permission_router() -> Router<PgPool> {
    Router::new().route("/permission", get(all_permission_handler))
}
