use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::api_key::create_api_key_handler;

pub fn api_key_router() -> Router<PgPool> {
    Router::new().route(
        "/organization/{org_id}/api_key",
        post(create_api_key_handler),
    )
}
