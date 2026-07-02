use axum::{Router, routing::post};
use sqlx::{Pool, Postgres};

use crate::handlers::organization::create;

pub fn organization_router() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/organization", post(create))
}