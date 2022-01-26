use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for `sea-orm`s Connection.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectionBuilder {
    /// The URI of the database
    pub url: String,
    /// Maximum number of connections for a pool
    pub max_connections: Option<u32>,
    /// Minimum number of connections for a pool
    pub min_connections: Option<u32>,
    /// The connection timeout for a packet connection
    pub connect_timeout: Option<Duration>,
    /// Maximum idle time for a particular connection
    pub idle_timeout: Option<Duration>,
}

impl ConnectionBuilder {
    /// Creates a new [ConnectionBuilder] with provided database url.
    pub fn new(url: String) -> Self {
        Self {
            url,
            max_connections: None,
            min_connections: None,
            connect_timeout: None,
            idle_timeout: None,
        }
    }

    /// Sets the maximum amount of connections to the database active at one time.
    pub fn max_connections(&mut self, value: Option<u32>) -> &mut Self {
        self.max_connections = value;
        self
    }

    /// Sets the minimum amount of connections to the database active at one time.
    pub fn min_connections(&mut self, value: Option<u32>) -> &mut Self {
        self.min_connections = value;
        self
    }

    /// Set the connection timeout for database connections.
    pub fn connect_timeout(&mut self, value: Option<Duration>) -> &mut Self {
        self.connect_timeout = value;
        self
    }

    /// Set the timeout after which a connections disconnects.
    pub fn idle_timeout(&mut self, value: Option<Duration>) -> &mut Self {
        self.idle_timeout = value;
        self
    }

    /// Connect to the database.
    #[instrument(
        name = "database_connect",
        level = "debug",
        skip(self),
        fields(url = %self.url, min_connections, max_connections, connect_timeout, idle_timeout)
    )]
    pub async fn connect(self) -> Result<DatabaseConnection, DbErr> {
        let mut options = ConnectOptions::from(self.url);

        if let Some(min_connections) = self.min_connections {
            tracing::Span::current().record("min_connections", &min_connections);
            options.min_connections(min_connections);
        }
        if let Some(max_connections) = self.max_connections {
            tracing::Span::current().record("max_connections", &max_connections);
            options.max_connections(max_connections);
        }
        if let Some(connect_timeout) = self.connect_timeout {
            tracing::Span::current()
                .record("connect_timeout", &tracing::field::debug(&connect_timeout));
            options.connect_timeout(connect_timeout);
        }
        if let Some(idle_timeout) = self.idle_timeout {
            tracing::Span::current().record("idle_timeout", &tracing::field::debug(&idle_timeout));
            options.idle_timeout(idle_timeout);
        }
        options.sqlx_logging(false);

        debug!("Connecting to database using provided options");
        Database::connect(options).await
    }
}
