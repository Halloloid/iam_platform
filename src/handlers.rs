use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::{
        auth_config::{AuthActor, AuthContext},
        response_config::AppError,
    },
    repositories::organization::check_permission,
};

pub mod api_key;
pub mod audit_logs;
pub mod health;
pub mod membership;
pub mod organization;
pub mod permission;
pub mod role;
pub mod session;
pub mod user;

pub async fn resolver_actor(
    auth: &AuthContext,
    required_permission: Option<&str>,
    org_id: Uuid,
    pool: &PgPool,
) -> Result<AuthActor, AppError> {
    let actor = AuthActor::from_auth(auth);

    match auth {
        AuthContext::User(claims) => {
            if let Some(permission) = required_permission
                && !check_permission(&pool, claims.sub, org_id, permission).await?
            {
                return Err(AppError::Forbidden);
            }
        }
        AuthContext::ApiKey(api_key_record) => {
            if api_key_record.org_id != org_id {
                return Err(AppError::Forbidden);
            }

            if let Some(permission) = required_permission
                && !api_key_record.scopes.contains(&permission.to_string())
            {
                return Err(AppError::Forbidden);
            }
        }
    }

    Ok(actor)
}

pub fn require_user(auth: &AuthContext) -> Result<Uuid, AppError> {
    match auth {
        AuthContext::User(claims) => return Ok(claims.sub),
        AuthContext::ApiKey(_) => return Err(AppError::Forbidden),
    }
}
