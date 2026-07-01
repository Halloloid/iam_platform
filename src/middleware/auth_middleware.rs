use axum::{extract::{Request, State}, middleware::Next, response::Response};
use sqlx::PgPool;

use crate::{config::{auth_config::verify_token, response_config::AppError}, repositories::session::validate_sessions};

pub async fn auth(
    State(pool) : State<PgPool>,
    mut req: Request,
    next: Next
) -> Result<Response, AppError> {

    let Some(header) = req
        .headers()
        .get(axum::http::header::AUTHORIZATION) else {
        return Err(AppError::Unauthorized);
    };

    let header = header.to_str().expect("Error in String Conversion");

    let Some(token) = header.strip_prefix("Bearer ") else {
        return Err(AppError::Unauthorized);
    };

    let Ok(claims) = verify_token(token) else {
        return Err(AppError::Unauthorized);
    };

    if let Ok(revoked) =  validate_sessions(claims.sid.clone(), &pool).await {
        if revoked {return Err(AppError::Unauthorized);}
    }
    
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
