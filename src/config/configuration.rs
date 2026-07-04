use config::{Config as ConfigBuilder, File};
use serde::Deserialize;

use super::{env, env_configuration::EnvConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub logger: LoggerConfig,
    pub cors: CorsConfig,
    pub pagination: PaginationConfig,
    pub upload: UploadConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub host: String,
    pub port: u16,
    pub timezone: String,
    pub debug: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub driver: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub acquire_timeout: u64,
    pub log_sql: bool,

    #[serde(skip)]
    pub host: String,

    #[serde(skip)]
    pub port: u16,

    #[serde(skip)]
    pub database: String,

    #[serde(skip)]
    pub username: String,

    #[serde(skip)]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub issuer: String,
    pub access_token_expired: u64,
    pub refresh_token_expired: u64,

    #[serde(skip)]
    pub secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggerConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationConfig {
    pub default_page: u32,
    pub default_limit: u32,
    pub max_limit: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UploadConfig {
    pub max_size: u64,
    pub allowed_extensions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allow_credentials: bool,
    pub allow_origin: String,
    pub allow_methods: Vec<String>,
    pub allow_headers: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        env::load();

        let mut config: Config = ConfigBuilder::builder()
            .add_source(File::with_name("configs/app"))
            .build()?
            .try_deserialize()?;

        let env: EnvConfig =
            envy::from_env().map_err(|e| config::ConfigError::Message(e.to_string()))?;

        config.database.host = env.mysql_host;
        config.database.port = env.mysql_port;
        config.database.database = env.mysql_database;
        config.database.username = env.mysql_username;
        config.database.password = env.mysql_password;

        config.jwt.secret = env.jwt_secret;

        Ok(config)
    }
}
