mod actions;
mod controller;

use axum::routing::post;
use axum::Router;

pub fn get_router() -> Router {
    Router::new()
        .route("/login", post(controller::login))
        .route("/logout", post(controller::logout))
        .route("/revoke", post(controller::revoke))
        .route("/refresh", post(controller::refresh))
}
