pub mod api_key;
pub mod audit_logs;
pub mod membership;
pub mod organization;
pub mod permission;
pub mod role;
pub mod session;
pub mod user;

pub mod service_helper {
    use crate::config::response_config::AppError;
    use base64::{Engine, engine::general_purpose};
    use chrono::{DateTime, Utc};

    pub fn encode_cursor(created_at: DateTime<Utc>) -> String {
        general_purpose::STANDARD.encode(created_at.to_rfc3339())
    }

    pub fn decode_cursor(cursor: &str) -> Result<DateTime<Utc>, AppError> {
        let bytes = general_purpose::STANDARD
            .decode(cursor)
            .map_err(|_| AppError::BadRequest(String::from("")))?;

        let s = String::from_utf8(bytes).map_err(|_| AppError::BadRequest(String::from("")))?;

        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|_| AppError::BadRequest(String::from("")))
    }
}
