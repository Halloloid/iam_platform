use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

use crate::handlers::user::{update_profile, view_profile};

pub fn user_router() -> Router<Pool<Postgres>> {
    Router::new().route("/user/me", get(view_profile).patch(update_profile))
}
