use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::{auth_config::AuthContext, response_config::AppError},
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

pub async fn resolver_user_id(
    auth: &AuthContext,
    required_permission: &str,
    org_id: Uuid,
    pool: PgPool,
) -> Result<Option<Uuid>, AppError> {
    match auth {
        AuthContext::User(claims) => {
            if !check_permission(&pool, claims.sub, org_id, required_permission).await? {
                return Err(AppError::Forbidden);
            }

            Ok(Some(claims.sub))
           
        }
        AuthContext::ApiKey(api_key_record) => {

            if api_key_record.org_id != org_id{
                return Err(AppError::Forbidden);
            }

            if !api_key_record.scopes.contains(&required_permission.to_string()){
                return Err(AppError::Forbidden);
            }
            Ok(None)
        },
    }
}
