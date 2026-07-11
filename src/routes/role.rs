use axum::{
    Router,
    routing::{patch, post},
};
use sqlx::{Pool, Postgres};

use crate::handlers::role::{
    all_roles_handler, create_role_handler, delete_role_handler, update_role_handler,
};

pub fn role_router() -> Router<Pool<Postgres>> {
    let role_id_router = Router::new().route(
        "/organization/{id}/role/{roleid}",
        patch(update_role_handler).delete(delete_role_handler),
    );
    Router::new()
        .route(
            "/organization/{id}/role",
            post(create_role_handler).get(all_roles_handler),
        )
        .merge(role_id_router)
}
