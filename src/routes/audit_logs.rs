use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::audit_logs::user_logs_handler;

pub fn audit_logs_router() -> Router<PgPool> {
    Router::new().route("/audit-logs", get(user_logs_handler))
}
