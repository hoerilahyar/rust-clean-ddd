use config::{Config as ConfigBuilder, Environment, File};
use serde::Deserialize;

use super::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub logger: LoggerConfig,
    pub cors: CorsConfig,
    pub pagination: PaginationConfig,
    pub upload: UploadConfig,
    pub storage: StorageConfig,
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

    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub database: i64,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub issuer: String,
    pub access_token_expired: u64,
    pub refresh_token_expired: u64,
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

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub provider: String,
    pub local_path: String,
    pub base_url: String,
    pub bucket: String,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        env::load();

        ConfigBuilder::builder()
            .add_source(File::with_name("configs/app"))
            .add_source(Environment::default().separator("_"))
            .build()?
            .try_deserialize()
    }
}
