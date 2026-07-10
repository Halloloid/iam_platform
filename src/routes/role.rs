use axum::{Router, routing::post};
use sqlx::{Pool, Postgres};

use crate::handlers::role::create_role_handler;

pub fn role_router() -> Router<Pool<Postgres>> {
    Router::new().route("/organization/{id}/role", post(create_role_handler))
}
