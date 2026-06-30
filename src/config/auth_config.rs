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

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn create_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    dotenvy::dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_Secret Not Found");

    let token = Claims {
        sub: user_id,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &token,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token:&str) -> Result<Claims,jsonwebtoken::errors::Error> {
    dotenvy::dotenv().ok();

    let secret = env::var("JWT_SECRET").expect("JWT_Secret Not Found");

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())?;

    Ok(token_data.claims)
}