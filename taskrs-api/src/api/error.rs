use crate::api::auth::error::AuthError;
use axum::response::{IntoResponse, Response};
use axum::Json;
use hyper::StatusCode;
use serde_json::json;
use taskrs_core::error::Error;
use taskrs_db::sea_orm::DbErr;

pub enum ApiError {
    Auth(AuthError),

    // External Errors
    Argon(Box<dyn std::error::Error>),
    Database(DbErr),
    JsonWebToken(Box<dyn std::error::Error>),
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}

impl From<taskrs_core::error::Error> for ApiError {
    fn from(err: taskrs_core::error::Error) -> Self {
        match err {
            Error::Argon(e) => Self::Argon(Box::new(e)),
            Error::Auth(_) => Self::Auth(AuthError::Credentials),
            Error::Database(e) => Self::Database(e),
            Error::JsonWebToken(e) => Self::JsonWebToken(Box::new(e)),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Auth(e) => e.into_response(),
            ApiError::Argon(e) => match cfg!(debug_assertions) {
                true => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Argon error: {}", e) })),
                )
                    .into_response(),
                false => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal Server Error"})),
                )
                    .into_response(),
            },
            ApiError::Database(e) => match cfg!(debug_assertions) {
                true => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Database error: {}", e) })),
                )
                    .into_response(),
                false => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal Server Error"})),
                )
                    .into_response(),
            },
            ApiError::JsonWebToken(e) => match cfg!(debug_assertions) {
                true => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("JsonWebToken error: {}", e) })),
                )
                    .into_response(),
                false => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal Server Error"})),
                )
                    .into_response(),
            },
        }
    }
}
