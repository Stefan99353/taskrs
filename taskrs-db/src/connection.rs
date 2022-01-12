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
    pub fn new(url: String) -> Self {
        Self {
            url,
            max_connections: None,
            min_connections: None,
            connect_timeout: None,
            idle_timeout: None,
        }
    }

    pub fn max_connections(&mut self, value: Option<u32>) -> &mut Self {
        self.max_connections = value;
        self
    }

    pub fn min_connections(&mut self, value: Option<u32>) -> &mut Self {
        self.min_connections = value;
        self
    }

    pub fn connect_timeout(&mut self, value: Option<Duration>) -> &mut Self {
        self.connect_timeout = value;
        self
    }

    pub fn idle_timeout(&mut self, value: Option<Duration>) -> &mut Self {
        self.idle_timeout = value;
        self
    }

    pub async fn connect(self) -> Result<DatabaseConnection, DbErr> {
        let mut options = ConnectOptions::from(self.url);

        if let Some(min_connections) = self.min_connections {
            options.min_connections(min_connections);
        }
        if let Some(max_connections) = self.max_connections {
            options.max_connections(max_connections);
        }
        if let Some(connect_timeout) = self.connect_timeout {
            options.connect_timeout(connect_timeout);
        }
        if let Some(idle_timeout) = self.idle_timeout {
            options.idle_timeout(idle_timeout);
        }
        options.sqlx_logging(false);

        Database::connect(options).await
    }
}
