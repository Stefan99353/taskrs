use crate::config::{Config, DatabaseConfig};
use crate::seeding::{
    seed_permissions, seed_root_role, seed_root_role_permissions, seed_root_user,
};
use axum::routing::IntoMakeService;
use axum::{Router, Server};
use http_body::combinators::UnsyncBoxBody;
use hyper::body::HttpBody as _;
use hyper::header::{AUTHORIZATION, CONTENT_LENGTH};
use hyper::http::HeaderValue;
use hyper::server::conn::AddrIncoming;
use hyper::Response;
use std::iter::once;
use std::net::SocketAddr;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use taskrs_db::connection::ConnectionBuilder;
use taskrs_db::sea_orm::DbConn;
use tower_http::cors::any;
use tower_http::trace::DefaultOnResponse;
use tower_http::{add_extension, compression, cors, sensitive_headers, set_header, trace};
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
    let router = crate::api::get_router()
        // Mark the `Authorization` request header as sensitive so it doesn't show in logs
        .layer(sensitive_headers::SetSensitiveHeadersLayer::new(once(
            AUTHORIZATION,
        )))
        // High level logging of requests and responses
        .layer(
            trace::TraceLayer::new_for_http()
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        // Compress responses
        .layer(compression::CompressionLayer::new())
        // CORS
        .layer(
            cors::CorsLayer::new()
                .allow_origin(any())
                .allow_methods(any())
                .allow_headers(any()),
        )
        // If the response has a known size set the `Content-Length` header
        .layer(set_header::SetResponseHeaderLayer::overriding(
            CONTENT_LENGTH,
            content_length_from_response,
        ))
        // Wrap application state for extraction
        .layer(add_extension::AddExtensionLayer::new(state))
        // Wrap database connection for extraction
        .layer(add_extension::AddExtensionLayer::new(Arc::new(db)));

    Server::bind(&address).serve(router.into_make_service())
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
    seed_permissions(db).await;

    // Seed root role and its permissions
    let role = seed_root_role(db).await;
    seed_root_role_permissions(role.id, db).await;

    // Seed root user
    if config.seeding.seed_root_user {
        let root_user = seed_root_user(
            config.seeding.root_user_email.clone(),
            config.seeding.root_user_password.clone(),
            config.seeding.root_user_first_name.clone(),
            config.seeding.root_user_last_name.clone(),
            db,
        )
        .await;

        // Grant root role
        if config.seeding.grant_root_role {
            taskrs_db::actions::access_control::add_user_roles(root_user.id, vec![role.id], db)
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
