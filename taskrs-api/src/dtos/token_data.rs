use crate::api::error::ApiError;
use crate::application::ApplicationState;
use axum::async_trait;
use axum::extract::{Extension, FromRequest, RequestParts, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessTokenData {
    pub iat: i64,
    pub exp: i64,
    pub user: taskrs_db::models::user::dtos::User,
}

impl AccessTokenData {
    pub fn new(user: taskrs_db::models::user::dtos::User, exp: u32) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + exp as i64,
            user,
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for AccessTokenData
where
    B: Send,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract state
        let Extension(state): Extension<ApplicationState> = Extension::from_request(req)
            .await
            .map_err(|err| err.into_response())?;

        // Extract the token from authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| ApiError::InvalidAccessToken.into_response())?;

        // Decode the data
        let token_data = jsonwebtoken::decode::<Self>(
            bearer.token(),
            &jsonwebtoken::DecodingKey::from_secret(
                state.config.authentication.access_token_secret.as_bytes(),
            ),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| ApiError::InvalidAccessToken.into_response())?;

        Ok(token_data.claims)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RefreshTokenData {
    pub iat: i64,
    pub exp: i64,
    pub user_id: i32,
}

impl RefreshTokenData {
    pub fn new(user_id: i32, exp: u32) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + exp as i64,
            user_id,
        }
    }
}
