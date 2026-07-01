use std::net::IpAddr;

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use sqlx::PgPool;

use crate::{
    config::response_config::AppError,
    models::user::{Create, LoginReq, LoginRes},
    services,
};

pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<Create>,
) -> Result<impl IntoResponse, AppError> {
    services::user::register(&pool, req).await?;

    Ok(StatusCode::CREATED)
}


pub async fn login(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<LoginReq>,
) -> Result<Json<LoginRes>, AppError> {
    let device = headers
        .get(header::USER_AGENT)
        .and_then(|x| x.to_str().ok())
        .unwrap_or("Unknown Device")
        .to_string();

    let ip: IpAddr = headers
        .get(header::FORWARDED)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .unwrap_or("0.0.0.0")
        .parse()
        .unwrap();

    let res = services::user::login(&pool, req, ip, device).await?;

    Ok(Json(res))
}
