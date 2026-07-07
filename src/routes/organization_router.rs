use axum::{Router, routing::post};
use sqlx::{Pool, Postgres};

use crate::handlers::organization::{all_orgs, create};

pub fn organization_router() -> Router<Pool<Postgres>> {
    Router::new().route("/organization", post(create).get(all_orgs))
}
