use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub seeding: SeedingConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logs: LogConfig,
}

impl Config {
    pub fn build() -> Self {
        dotenv::dotenv().ok();
        let mut config = config::Config::default();

        // Default
        let default_config = config::Config::try_from(&Config::default())
            .expect("Error loading default configuration");
        config
            .merge(default_config)
            .expect("Error loading default configuration");

        let config_file = match std::env::var("TASKRS_CONFIG_PATH") {
            Ok(path) => format!("{}/taskrs", path),
            Err(_) => "./taskrs".to_string(),
        };
        config
            .merge(config::File::with_name(&config_file).required(false))
            .expect("Error loading file configuration");

        // Environment
        config
            .merge(config::Environment::with_prefix("TASKRS"))
            .expect("Error loading environment configuration");

        config.try_into().expect("Error loading configuration")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SeedingConfig {
    pub root_user_email: String,
    pub root_user_password: String,
    pub root_user_first_name: Option<String>,
    pub root_user_last_name: Option<String>,
    pub seed_root_user: bool,
    pub grant_root_role: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub bind_address: IpAddr,
    pub bind_port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub connect_timeout: u32, // Seconds
    pub idle_timeout: u32,    // Seconds
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogConfig {
    pub log_to_file: bool,
    pub log_to_stdout: bool,
    pub log_dir: String,
    pub log_prefix: String,
    pub rust_log: String,
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
    fn default() -> Self {
        Self {
            seeding: SeedingConfig::default(),
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            logs: LogConfig::default(),
        }
    }
}

impl Default for SeedingConfig {
    fn default() -> Self {
        Self {
            root_user_email: "root@taskrs.com".to_string(),
            root_user_password: "root".to_string(),
            root_user_first_name: None,
            root_user_last_name: None,
            seed_root_user: true,
            grant_root_role: true,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:./taskrs.db?mode=rwc".to_string(),
            min_connections: 5,
            max_connections: 100,
            connect_timeout: 10,
            idle_timeout: 10,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: IpAddr::from([0, 0, 0, 0]),
            bind_port: 8080,
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_to_file: false,
            log_to_stdout: true,
            log_dir: "logs".to_string(),
            log_prefix: "taskrs.log".to_string(),
            rust_log: "info".to_string(),
        }
    }
}
