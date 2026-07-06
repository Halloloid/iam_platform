use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bcrypt::BcryptError;
use validator::ValidationErrors;

pub enum AppError {
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict(String),
    Database,
    InternalServerError,
    PassWordHashErr(BcryptError),
    Validation(ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, Json(msg)).into_response(),
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, Json("Unauthorized")).into_response()
            }
            AppError::Forbidden => (StatusCode::FORBIDDEN, Json("Forbidden")).into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, Json("Not Found")).into_response(),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, Json(msg)).into_response(),
            AppError::Database => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json("Data Base Error")).into_response()
            }
            AppError::PassWordHashErr(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Password hash Error"),
            )
                .into_response(),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal Server Error"),
            )
                .into_response(),
            AppError::Validation(validation_errors) => {
                (StatusCode::BAD_REQUEST, Json(validation_errors)).into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        AppError::Database
    }
}

impl From<BcryptError> for AppError {
    fn from(err: BcryptError) -> Self {
        AppError::PassWordHashErr(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized
    }
}
