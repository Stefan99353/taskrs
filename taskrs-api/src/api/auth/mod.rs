// mod actions;
mod controller;
pub mod error;

use axum::routing::post;
use axum::Router;

pub const ACCESS_TOKEN_COOKIE: &str = "access_token";
pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

pub fn get_router() -> Router {
    Router::new()
        .route("/login", post(controller::login))
        .route("/logout", post(controller::logout))
        .route("/revoke", post(controller::revoke))
}
