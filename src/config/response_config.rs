use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict(String),
    Database(sqlx::Error),
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, Json(msg)).into_response(),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED,Json("Unauthorized")).into_response(),
            AppError::Forbidden => (StatusCode::FORBIDDEN,Json("Forbidden")).into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND,Json("Not Found")).into_response(),
            AppError::Conflict(msg) => (StatusCode::CONFLICT,Json(msg)).into_response(),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR,Json("Data Base Error")).into_response(),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR,Json("Internal Server Error")).into_response(),
        }
    }
}


impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}