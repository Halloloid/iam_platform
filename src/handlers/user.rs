use std::net::IpAddr;

use axum::{
    Extension, Json, extract::State, http::{HeaderMap, StatusCode, header}, response::IntoResponse,
};
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::{
    config::{auth_config::Claims, response_config::AppError}, models::{
        session::ReqToken, user::{Create, LoginReq, LoginRes, UpdateProfile},
    }, repositories::{session::revoke_session, user::{fnd_by_user_id, update_user}}, services,
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

pub async fn refresh(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(req): Json<ReqToken>,
) -> Result<Json<LoginRes>, AppError> {
    let ip: IpAddr = headers
        .get(header::FORWARDED)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .unwrap_or("0.0.0.0")
        .parse()
        .unwrap();

    let res = services::user::refresh(&pool, req.refresh_token, ip).await?;

    Ok(Json(res))
}

pub async fn logout(
    State(pool): State<PgPool>,
    Json(req): Json<ReqToken>,
) -> Result<Json<Value>, AppError> {
    revoke_session(&pool, req.refresh_token).await?;

    Ok(
        Json(
            json!(
            {
                "logout":"SuccessFull"
            }
            )
        )
    )
}


pub async fn view_profile(
    Extension(claims) : Extension<Claims>,
    State(pool): State<PgPool>
) -> Result<impl IntoResponse,AppError>{
    let user_id = claims.sub;

    let user = fnd_by_user_id(&pool, user_id).await?;

    Ok(Json(user))
}

pub async fn update_profile(
    Extension(claims) : Extension<Claims>,
    State(pool): State<PgPool>,
    Json(name): Json<UpdateProfile>
) -> Result<impl IntoResponse,AppError>{
    let user_id = claims.sub;

    update_user(&pool, user_id, name.name).await?;

    Ok(Json(json!({"updation":"SuccessFull"})))
}