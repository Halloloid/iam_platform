use std::env;

use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::response_config::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let pass = hash(password, DEFAULT_COST)?;
    Ok(pass)
}

pub fn verify_password(password: &str, hashed_passwrd: &str) -> Result<bool, AppError> {
    let verify = verify(password, hashed_passwrd)?;
    Ok(verify)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn create_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    dotenvy::dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_Secret Not Found");

    let token = Claims {
        sub: user_id,
        exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &token,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    dotenvy::dotenv().ok();

    let secret = env::var("JWT_SECRET").expect("JWT_Secret Not Found");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[derive(Clone)]
pub struct ApiKeyRecord {
    pub id: Uuid,
    pub org_id: Uuid,
    pub scopes: Vec<String>,
}

#[derive(Clone)]
pub enum AuthContext {
    User(Claims),
    ApiKey(ApiKeyRecord),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());
    //---Password Tests---
    #[test]
    fn test_hashed_password_return_hash() {
        let hash = hash_password("mysecrectpassword").unwrap();
        assert_ne!(hash, "mysecrectpassword");
    }

    #[test]
    fn test_verify_correct_password() {
        let hash = hash_password("mysecrectpassword").unwrap();
        let result = verify_password("mysecrectpassword", &hash).unwrap();
        assert!(result);
    }

    #[test]
    fn test_verify_wrong_password() {
        let hash = hash_password("mysecrectpassword").unwrap();
        let result = verify_password("wrongpassword", &hash).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_same_password_produces_different_hash() {
        let hash1 = hash_password("mysecrectpassword").unwrap();
        let hash2 = hash_password("mysecrectpassword").unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_empty_password_still_hashes() {
        let result = hash_password("");
        assert!(result.is_ok());
    }

    //===JWT Tests====
    #[test]
    fn test_create_and_verify_token() {
        let key = "JWT_SECRET";
        let _lock = ENV_LOCK.lock().unwrap();

        unsafe {
            std::env::set_var(key, "test-secrect-key");
        }

        let user_id = Uuid::new_v4();

        let token = create_token(user_id).unwrap();
        let claims = verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_verify_invalid_token_fails() {
        let key = "JWT_SECRET";
        let _lock = ENV_LOCK.lock().unwrap();

        unsafe {
            std::env::set_var(key, "test-secrect-key");
        }

        let res = verify_token("this.is.not.a.valid.token");

        assert!(res.is_err());
    }

    #[test]
    fn test_verify_tamperd_token_fails() {
        let key = "JWT_SECRET";
        let _lock = ENV_LOCK.lock().unwrap();

        unsafe {
            std::env::set_var(key, "test-secrect-key");
        }

        let user_id = Uuid::new_v4();
        let token = create_token(user_id).unwrap();

        let tamperd = format!("{}extra", token);
        let res = verify_token(&tamperd);

        assert!(res.is_err());
    }

    #[test]
    fn test_token_with_wrong_secrect_fails() {
        let key = "JWT_SECRET";
        let _lock = ENV_LOCK.lock().unwrap();

        unsafe {
            std::env::set_var(key, "test-secrect-key1");
        }

        let user_id = Uuid::new_v4();
        let token = create_token(user_id).unwrap();

        unsafe {
            std::env::set_var(key, "test-secrect-key2");
        }

        let res = verify_token(&token);
        assert!(res.is_err());
    }
}
