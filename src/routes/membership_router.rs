use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::membership::add_member_handler;

pub fn membership_router() -> Router<PgPool> {
    Router::new().route("/organization/{org_id}/member", post(add_member_handler))
}
