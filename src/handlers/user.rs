use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

use crate::{config::response_config::AppError, models::user::Create, services};

pub async fn register(
    State(pool):State<PgPool>,
    Json(req) : Json<Create>
) -> Result<impl IntoResponse,AppError> {

    services::user::register(&pool, req).await?;

    Ok(StatusCode::CREATED)
}