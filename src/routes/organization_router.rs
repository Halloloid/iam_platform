use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};

use crate::handlers::organization::{all_orgs, create, paticular_org};

pub fn organization_router() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/organization", post(create).get(all_orgs))
        .route("/organization/{id}", get(paticular_org))
}
