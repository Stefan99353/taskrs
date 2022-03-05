use crate::api::auth::error::AuthError;
use crate::api::auth::{ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE};
use crate::api::error::ApiError;
use crate::application::ApplicationState;
use axum::async_trait;
use axum::extract::{Extension, FromRequest, RequestParts};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use taskrs_core::logic::ReadModelTrait;
use taskrs_core::models::auth::{AccessTokenData, RefreshTokenData};
use taskrs_core::models::refresh_token::RefreshToken;
use taskrs_core::models::user::User;
use taskrs_db::sea_orm::sea_query::IntoCondition;
use taskrs_db::sea_orm::{ColumnTrait, DbConn};
use time::Duration;
use tower_cookies::{Cookie, Cookies};
use tracing::field;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Requester {
    pub id: i32,
    pub email: String,
}

impl From<User> for Requester {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for Requester
where
    B: Send,
{
    type Rejection = Response;

    #[instrument(
        name = "extract_requester",
        level = "debug",
        skip_all,
        ret,
        fields(requester_id, requester_email)
    )]
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        trace!("Extracting state");
        let Extension(state): Extension<ApplicationState> = Extension::from_request(req)
            .await
            .map_err(|err| err.into_response())?;

        trace!("Extracting cookies");
        let cookies = Cookies::from_request(req)
            .await
            .map_err(|err| err.into_response())?;
        trace!("Getting access token");
        let access_token_cookie = cookies
            .get(ACCESS_TOKEN_COOKIE)
            .ok_or_else(|| AuthError::AccessToken.into_response())?;

        trace!("Validate access token");
        let (token_data_result, expired) =
            taskrs_core::logic::auth::validate_token::<AccessTokenData>(
                access_token_cookie.value(),
                state.config.authentication.access_token_secret.as_bytes(),
            );

        // Valid access token
        if let Ok(token_data) = token_data_result {
            debug!("Valid access token");
            let requester: Requester = token_data.claims.user.into();
            tracing::Span::current().record("requester_id", &field::display(requester.id));
            tracing::Span::current().record("requester_email", &field::display(&requester.email));
            return Ok(requester);
        }

        // Valid access token but expired
        if expired {
            debug!("Expired access token");
            trace!("Extracting database");
            let Extension(db): Extension<Arc<DbConn>> = Extension::from_request(req)
                .await
                .map_err(|err| err.into_response())?;

            trace!("Getting refresh token");
            let refresh_token_cookie = cookies
                .get(REFRESH_TOKEN_COOKIE)
                .ok_or_else(|| AuthError::RefreshToken.into_response())?;

            trace!("Check database for refresh token");
            let _db_refresh_token = RefreshToken::find_one(
                taskrs_db::models::refresh_token::Column::Token
                    .eq(refresh_token_cookie.value())
                    .into_condition(),
                db.as_ref(),
            )
            .await
            .map_err(|e| ApiError::Database(e).into_response())?
            .ok_or_else(|| AuthError::RefreshToken.into_response())?;

            trace!("Validate refresh token");
            let (token_data_result, _) = taskrs_core::logic::auth::validate_token::<RefreshTokenData>(
                refresh_token_cookie.value(),
                state.config.authentication.refresh_token_secret.as_bytes(),
            );

            let user_id = token_data_result
                .map_err(|_| AuthError::RefreshToken.into_response())?
                .claims
                .user_id;

            trace!("Get user from database");
            let user = User::get(user_id, db.as_ref())
                .await
                .map_err(|e| ApiError::Database(e).into_response())?
                .ok_or_else(|| AuthError::User.into_response())?;

            tracing::Span::current().record("requester_id", &field::display(user.id));
            tracing::Span::current().record("requester_email", &field::display(&user.email));

            trace!("Check if user is enabled");
            if !user.enabled {
                return Err(AuthError::User.into_response());
            }

            trace!("Generate new access token");
            let access_token_data = AccessTokenData::new(
                user.clone(),
                state.config.authentication.access_token_expiration_time,
            );
            let access_token = taskrs_core::logic::auth::encode_token(
                &access_token_data,
                state.config.authentication.access_token_secret.as_bytes(),
            )
            .map_err(|e| ApiError::JsonWebToken(Box::new(e)).into_response())?;

            // Set cookie
            let access_exp = state.config.authentication.access_token_expiration_time as i64;
            cookies.add(
                Cookie::build(ACCESS_TOKEN_COOKIE, access_token)
                    .max_age(Duration::seconds(access_exp))
                    .http_only(true)
                    .finish(),
            );

            // Return requester
            return Ok(user.into());
        }

        // Invalid access token
        debug!("Invalid access token");
        Err(AuthError::AccessToken.into_response())
    }
}
