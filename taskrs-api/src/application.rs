use crate::config::{Config, DatabaseConfig, ServerConfig};
use axum::routing::{get_service, IntoMakeService};
use axum::{Json, Router, Server};
use http_body::combinators::UnsyncBoxBody;
use hyper::body::HttpBody as _;
use hyper::header::{AUTHORIZATION, CONTENT_LENGTH};
use hyper::http::HeaderValue;
use hyper::server::conn::AddrIncoming;
use hyper::{Response, StatusCode};
use serde_json::json;
use std::iter::once;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use taskrs_core::seeding::{
    seed_permissions, seed_root_role, seed_root_role_permissions, seed_root_user,
};
use taskrs_db::connection::ConnectionBuilder;
use taskrs_db::sea_orm::DbConn;
use tower_cookies::CookieManagerLayer;
use tower_http::add_extension::AddExtensionLayer;
#[cfg(not(debug_assertions))]
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::{EnvFilter, Registry};

pub struct Application {
    server: Server<AddrIncoming, IntoMakeService<Router>>,
    address: SocketAddr,
}

#[derive(Clone, Debug)]
pub struct ApplicationState {
    pub config: Config,
    pub log_reload_handle: Handle<EnvFilter, Registry>,
}

impl Application {
    #[instrument(name = "application_build", skip_all)]
    pub async fn build(config: Config, log_reload_handle: Handle<EnvFilter, Registry>) -> Self {
        debug!("Building application and connect to database");

        let db_connection = get_database_connection(&config.database).await;
        setup_database(&config, &db_connection).await;

        let bind_address = SocketAddr::new(config.server.bind_address, config.server.bind_port);
        let state = ApplicationState {
            config,
            log_reload_handle,
        };
        let server = build_server(bind_address, state, db_connection);

        Self {
            server,
            address: bind_address,
        }
    }

    #[instrument(name = "application", skip_all)]
    pub async fn run(self) -> hyper::Result<()> {
        info!("Start listening on {}", &self.address);
        self.server.await
    }
}

fn build_server(
    address: SocketAddr,
    state: ApplicationState,
    db: DbConn,
) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let router = build_router(&state.config.server)
        // Mark the `Authorization` request header as sensitive so it doesn't show in logs
        .layer(SetSensitiveHeadersLayer::new(once(AUTHORIZATION)))
        // High level logging of requests and responses
        .layer(TraceLayer::new_for_http().on_response(DefaultOnResponse::new().level(Level::INFO)))
        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        // If the response has a known size set the `Content-Length` header
        .layer(SetResponseHeaderLayer::overriding(
            CONTENT_LENGTH,
            content_length_from_response,
        ))
        // Manager Layer for cookies
        .layer(CookieManagerLayer::new())
        // Wrap application state for extraction
        .layer(AddExtensionLayer::new(state))
        // Wrap database connection for extraction
        .layer(AddExtensionLayer::new(Arc::new(db)));

    // Compress responses only in release mode
    #[cfg(not(debug_assertions))]
    let router = router.layer(CompressionLayer::new());

    Server::bind(&address).serve(router.into_make_service())
}

fn build_router(server_config: &ServerConfig) -> Router {
    let mut index_path = PathBuf::from(&server_config.spa_path);
    index_path.push(&server_config.spa_index);

    Router::new()
        .nest("/api", crate::api::get_api_router())
        .nest(
            "/static",
            get_service(
                ServeDir::new(&server_config.spa_path)
                    .precompressed_br()
                    .precompressed_gzip()
                    .precompressed_deflate(),
            )
            .handle_error(|_: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal Server Error"})),
                )
            }),
        )
        .fallback(
            get_service(
                ServeFile::new(&index_path)
                    .precompressed_br()
                    .precompressed_gzip()
                    .precompressed_deflate(),
            )
            .handle_error(|_: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal Server Error"})),
                )
            }),
        )
}

async fn get_database_connection(database_config: &DatabaseConfig) -> DbConn {
    debug!("Building connection options from config");
    let builder = ConnectionBuilder::new(database_config.url.clone())
        .max_connections(Some(database_config.max_connections))
        .min_connections(Some(database_config.min_connections))
        .connect_timeout(Some(Duration::from_secs(
            database_config.connect_timeout as u64,
        )))
        .idle_timeout(Some(Duration::from_secs(
            database_config.idle_timeout as u64,
        )))
        .to_owned();

    debug!("Connecting to database");
    builder.connect().await.unwrap_or_else(|err| {
        error!("Error while connecting to database and building connection pool.");
        error!("{}", err);
        exit(-1);
    })
}

async fn setup_database(config: &Config, db: &DbConn) {
    // Migrations
    let migrations = taskrs_db::migrations::Migrations::new(None);
    migrations.run(db).await.unwrap_or_else(|err| {
        error!("Error while running database migrations.");
        error!("{}", err);
        exit(-1);
    });

    // Seeding permissions
    seed_permissions(db).await.unwrap_or_else(|err| {
        error!("Error while seeding permissions.");
        error!("{}", err);
        exit(-1);
    });

    // Seed root role and its permissions
    let role = seed_root_role(db).await.unwrap_or_else(|err| {
        error!("Error while seeding root role.");
        error!("{}", err);
        exit(-1);
    });
    seed_root_role_permissions(role.id, db)
        .await
        .unwrap_or_else(|err| {
            error!("Error while seeding root role permissions.");
            error!("{}", err);
            exit(-1);
        });

    // Seed root user
    if config.seeding.seed_root_user {
        let root_user = seed_root_user(
            config.seeding.root_user_email.clone(),
            config.seeding.root_user_password.clone(),
            config.seeding.root_user_first_name.clone(),
            config.seeding.root_user_last_name.clone(),
            db,
        )
        .await
        .unwrap_or_else(|err| {
            error!("Error while seeding root user.");
            error!("{}", err);
            exit(-1);
        });

        // Grant root role
        if config.seeding.grant_root_role {
            taskrs_core::models::user::User::grant_roles(root_user.id, vec![role.id], db)
                .await
                .unwrap_or_else(|err| {
                    error!("Database error while granting root role: {}", err);
                    exit(-1);
                });
        }
    }
}

fn content_length_from_response(
    response: &Response<UnsyncBoxBody<axum::body::Bytes, axum::Error>>,
) -> Option<HeaderValue> {
    response
        .body()
        .size_hint()
        .exact()
        .map(|size| HeaderValue::from_str(&size.to_string()).unwrap())
}
