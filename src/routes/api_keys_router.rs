use axum::{
    Router,
    routing::{delete, post},
};
use sqlx::PgPool;

use crate::handlers::api_key::{
    all_api_keys_handler, create_api_key_handler, delete_api_key_handler,
};

pub fn api_key_router() -> Router<PgPool> {
    Router::new()
        .route(
            "/organization/{org_id}/api_key",
            post(create_api_key_handler).get(all_api_keys_handler),
        )
        .route(
            "/organization/{org_id}/api_key/{api_id}",
            delete(delete_api_key_handler),
        )
}
