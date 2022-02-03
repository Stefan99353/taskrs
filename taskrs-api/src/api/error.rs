use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use taskrs_db::sea_orm::DbErr;

#[derive(Debug)]
pub enum ApiError {
    Database(DbErr),
    Argon(taskrs_db::argon2::Error),
    Jwt(jsonwebtoken::errors::Error),
    MissingCredentials,
    WrongCredentials,
    InvalidAccessToken,
    InvalidRefreshToken,
    MissingRefreshToken,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(e) => {
                error!("{}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            ApiError::Argon(e) => {
                error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Password hashing/verifying error",
                )
            }
            ApiError::Jwt(e) => {
                error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error while creating/decoding JWTs",
                )
            }
            ApiError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            ApiError::WrongCredentials => (
                StatusCode::BAD_REQUEST,
                "Wrong credentials or user disabled",
            ),
            ApiError::InvalidAccessToken => {
                (StatusCode::UNAUTHORIZED, "Invalid bearer token provided")
            }
            ApiError::InvalidRefreshToken => {
                (StatusCode::BAD_REQUEST, "Refresh token is not valid")
            }
            ApiError::MissingRefreshToken => (StatusCode::BAD_REQUEST, "Refresh token is missing"),
        };

        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
