use axum::{
    Router,
    routing::{delete, get},
};
use sqlx::PgPool;

use crate::handlers::session::{list_session_handler, revoke_session_handler};

pub fn session_router() -> Router<PgPool> {
    Router::new()
        .route("/session", get(list_session_handler))
        .route("/session/{session_id}", delete(revoke_session_handler))
}
