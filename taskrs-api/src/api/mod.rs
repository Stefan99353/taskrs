mod auth;
pub mod error;
mod requester;

use axum::routing::get;
use axum::Router;

pub fn get_api_router() -> Router {
    Router::new()
        .route("/status", get(status))
        .nest("/auth", auth::get_router())
}

async fn status() -> &'static str {
    "System Online"
}
