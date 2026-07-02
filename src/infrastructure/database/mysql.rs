use std::time::Duration;

use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::config::Config;

pub async fn connect(config: &Config) -> Result<MySqlPool, sqlx::Error> {
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.database,
    );

    MySqlPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .acquire_timeout(Duration::from_secs(config.database.acquire_timeout))
        .idle_timeout(Duration::from_secs(config.database.idle_timeout))
        .max_lifetime(Duration::from_secs(config.database.max_lifetime))
        .connect(&database_url)
        .await
}
