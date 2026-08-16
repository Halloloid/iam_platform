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

#[cfg(test)]
mod tests {
    use crate::services::service_helper::{decode_cursor, encode_cursor};
    use base64::{Engine, engine::general_purpose};
    use chrono::{Timelike, Utc};

    #[test]
    fn test_encode_decode_cursor_roundtrip() {
        let now = Utc::now();
        let now = now.with_nanosecond(0).unwrap();

        let encode = encode_cursor(now);
        let decode = decode_cursor(&encode).unwrap();

        assert_eq!(now, decode);
    }

    #[test]
    fn test_decode_invalid_cursor_fails() {
        let res = decode_cursor("this is not base_64");

        assert!(res.is_err());
    }

    #[test]
    fn test_decode_valid_base64_but_invalid_date_fails() {
        let encode = general_purpose::STANDARD.encode("not-a-date");
        let res = decode_cursor(&encode);
        assert!(res.is_err());
    }

    #[test]
    fn test_different_timestamps_produce_different_cursors() {
        let t1 = Utc::now();
        let t2 = t1 + chrono::Duration::seconds(1);

        let c1 = encode_cursor(t1);
        let c2 = encode_cursor(t2);

        assert_ne!(c1, c2);
    }
}
