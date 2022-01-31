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
        };

        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
