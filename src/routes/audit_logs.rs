use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::audit_logs::{org_logs_handler, user_logs_handler};

pub fn audit_logs_router() -> Router<PgPool> {
    Router::new()
        .route("/audit-logs", get(user_logs_handler))
        .route("/organization/{org_id}/audit-logs", get(org_logs_handler))
}
