use crate::api::auth::error::AuthError;
use crate::api::auth::{ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE};
use crate::api::error::ApiError;
use crate::api::requester::Requester;
use crate::application::ApplicationState;
use axum::extract::Extension;
use axum::Json;
use std::sync::Arc;
use taskrs_core::models::auth::{Auth, Token};
use taskrs_db::sea_orm::DbConn;
use time::Duration;
use tower_cookies::{Cookie, Cookies};

#[instrument(
    name = "login",
    level = "debug",
    skip_all,
    fields (
        email = %login_data.email
    )
)]
pub async fn login(
    Json(login_data): Json<Auth>,
    cookies: Cookies,
    Extension(db): Extension<Arc<DbConn>>,
    Extension(state): Extension<ApplicationState>,
) -> Result<(), ApiError> {
    debug!("Check that credentials are provided");
    if login_data.email.is_empty() || login_data.password.is_empty() {
        return Err(AuthError::Credentials.into());
    }

    debug!("Generate access and refresh tokens");
    let config = state.config.authentication.clone().into_settings();
    let tokens = login_data.login(&config, db.as_ref()).await?;

    cookies.add(
        Cookie::build(ACCESS_TOKEN_COOKIE, tokens.access_token)
            .http_only(true)
            .path("/")
            .finish(),
    );

    let refresh_exp = state.config.authentication.refresh_token_expiration_time as i64;
    cookies.add(
        Cookie::build(REFRESH_TOKEN_COOKIE, tokens.refresh_token)
            .max_age(Duration::seconds(refresh_exp))
            .http_only(true)
            .path("/")
            .finish(),
    );

    Ok(())
}

#[instrument(name = "logout", level = "debug", skip_all)]
pub async fn logout(
    cookies: Cookies,
    Extension(db): Extension<Arc<DbConn>>,
) -> Result<(), ApiError> {
    let token = cookies
        .get(REFRESH_TOKEN_COOKIE)
        .ok_or(AuthError::RefreshToken)?;

    debug!("Invalidating refresh token");
    let count = Auth::logout(token.value(), db.as_ref()).await?;

    if count == 0 {
        Err(AuthError::RefreshToken.into())
    } else {
        cookies.remove(
            Cookie::build(ACCESS_TOKEN_COOKIE, "")
                .max_age(Duration::seconds(0))
                .http_only(true)
                .finish(),
        );
        cookies.remove(
            Cookie::build(REFRESH_TOKEN_COOKIE, "")
                .max_age(Duration::seconds(0))
                .http_only(true)
                .finish(),
        );
        Ok(())
    }
}

#[instrument(
    name = "logout",
    level = "debug",
    skip_all,
    fields (
        requester = requester.id
    )
)]
pub async fn revoke(
    requester: Requester,
    Json(token): Json<Token>,
    Extension(db): Extension<Arc<DbConn>>,
) -> Result<(), ApiError> {
    // TODO: Check permission of requester

    let count = Auth::logout(&token.token, db.as_ref()).await?;

    if count == 0 {
        Err(AuthError::RevokeRefreshToken.into())
    } else {
        Ok(())
    }
}
