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

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest(arg0) => f.debug_tuple("BadRequest").field(arg0).finish(),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::Forbidden => write!(f, "Forbidden"),
            Self::NotFound => write!(f, "NotFound"),
            Self::Conflict(arg0) => f.debug_tuple("Conflict").field(arg0).finish(),
            Self::Database => write!(f, "Database"),
            Self::InternalServerError => write!(f, "InternalServerError"),
            Self::PassWordHashErr(arg0) => f.debug_tuple("PassWordHashErr").field(arg0).finish(),
            Self::Validation(arg0) => f.debug_tuple("Validation").field(arg0).finish(),
        }
    }
}
