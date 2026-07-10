use axum::{Router, routing::post};
use sqlx::{Pool, Postgres};

use crate::handlers::role::{all_roles_handler, create_role_handler};

pub fn role_router() -> Router<Pool<Postgres>> {
    Router::new().route("/organization/{id}/role", post(create_role_handler).get(all_roles_handler))
}
