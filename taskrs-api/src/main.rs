#[macro_use]
extern crate tracing;

mod api;
mod application;
mod config;
pub mod dtos;
mod logging;
mod permissions;
mod seeding;

#[tokio::main]
async fn main() {
    // Config
    let config = config::Config::build();

    // Logging
    let (subscriber, log_reload_handle, _guards) = logging::get_subscriber(&config.logs);
    logging::init_subscriber(subscriber);

    // Application
    let application = application::Application::build(config, log_reload_handle).await;
    application.run().await.expect("Error while running server");
}
