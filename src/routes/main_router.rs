use axum::{Router,routing::get};

use crate::handlers::health;

pub fn main_router() -> Router{
    Router::new()
        .route("/health", get(health::health))
}