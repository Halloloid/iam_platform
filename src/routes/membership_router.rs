use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;

use crate::handlers::membership::{
    add_member_handler, all_members_handler, assign_role_handler, disassign_role_handler,
    remove_member_handler, return_role_of_member_handler,
};

pub fn membership_router() -> Router<PgPool> {
    Router::new()
        .route(
            "/organization/{org_id}/member",
            post(add_member_handler).get(all_members_handler),
        )
        .route(
            "/organization/{org_id}/member/{member_id}",
            delete(remove_member_handler),
        )
        .route(
            "/organization/{org_id}/member/{member_id}/role",
            get(return_role_of_member_handler).post(assign_role_handler),
        )
        .route(
            "/organization/{org_id}/member/{member_id}/role/{role_id}",
            delete(disassign_role_handler),
        )
}
