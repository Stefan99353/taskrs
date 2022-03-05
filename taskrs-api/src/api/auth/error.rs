use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    AccessToken,
    RefreshToken,
    Credentials,
    User,
    RevokeRefreshToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::AccessToken => {
                (StatusCode::UNAUTHORIZED, "Invalid access token".to_string())
            }
            AuthError::RefreshToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid refresh token".to_string(),
            ),
            AuthError::Credentials => (StatusCode::BAD_REQUEST, "Invalid credentials".to_string()),
            AuthError::User => (
                StatusCode::BAD_REQUEST,
                "User removed or disabled".to_string(),
            ),
            AuthError::RevokeRefreshToken => {
                (StatusCode::BAD_REQUEST, "Invalid refresh token".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
