use super::actions;
use crate::api::error::ApiError;
use crate::application::ApplicationState;
use crate::dtos::login_user::LoginUserDto;
use crate::dtos::token::TokenDto;
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use taskrs_db::sea_orm::DbConn;
use time::Duration;
use tower_cookies::{Cookie, Cookies};

const REFRESH_COOKIE_NAME: &str = "refresh_token";

pub async fn login(
    Json(login_data): Json<LoginUserDto>,
    Extension(db): Extension<Arc<DbConn>>,
    Extension(state): Extension<ApplicationState>,
    cookies: Cookies,
) -> Result<Json<TokenDto>, ApiError> {
    if login_data.email.is_empty() || login_data.password.is_empty() {
        return Err(ApiError::MissingCredentials);
    }

    let (access_token, refresh_token) =
        actions::login(login_data, &state.config.authentication, db.as_ref()).await?;

    let refresh_exp = state.config.authentication.refresh_token_expiration_time as i64;
    cookies.add(
        Cookie::build(REFRESH_COOKIE_NAME, refresh_token)
            .max_age(Duration::seconds(refresh_exp))
            .finish(),
    );

    Ok(Json(TokenDto {
        token: access_token,
    }))
}

pub async fn logout() -> impl IntoResponse {}

pub async fn refresh() -> impl IntoResponse {}

pub async fn revoke() -> impl IntoResponse {}
