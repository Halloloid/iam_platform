use axum::{extract::{Request, State}, middleware::Next, response::Response};
use sqlx::PgPool;

use crate::{config::{auth_config::{AuthContext, verify_token}, response_config::AppError}, repositories::{api_key::validate_api_key, session::validate_sessions}};

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

    let Ok(header) = header.to_str() else {
        return Err(AppError::Unauthorized);
    };

    let Some(token) = header.strip_prefix("Bearer ") else {
        return Err(AppError::Unauthorized);
    };

    if token.starts_with("iam_"){
        let Ok(record) = validate_api_key(token, &pool).await else {
            return Err(AppError::Database);
        };

        req.extensions_mut().insert(AuthContext::ApiKey(record));
    }else {
        let Ok(claims) = verify_token(token) else {
            return Err(AppError::Unauthorized);
        };
    
        if let Ok(revoked) =  validate_sessions(claims.sid.clone(), &pool).await {
            if revoked {return Err(AppError::Unauthorized);}
        }
        
        req.extensions_mut().insert(claims);
    }

    Ok(next.run(req).await)
}
