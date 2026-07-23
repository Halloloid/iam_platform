use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::session::list_session_handler;

pub fn session_router() -> Router<PgPool> {
    Router::new().route("/session", get(list_session_handler))
}
