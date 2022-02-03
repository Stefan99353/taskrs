mod auth;
pub mod error;

use axum::routing::get;
use axum::Router;

pub fn get_router() -> Router {
    let api_router = Router::new()
        .route("/status", get(status))
        .nest("/auth", auth::get_router());

    Router::new().nest("/api", api_router)
}

async fn status() -> &'static str {
    "System Online"
}
