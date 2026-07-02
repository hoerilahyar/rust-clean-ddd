use redis::{Client, aio::ConnectionManager};

use crate::config::Config;

pub async fn connect(config: &Config) -> Result<ConnectionManager, redis::RedisError> {
    let redis_url = if config.redis.password.is_empty() {
        format!(
            "redis://{}:{}/{}",
            config.redis.host, config.redis.port, config.redis.database,
        )
    } else {
        format!(
            "redis://:{}@{}:{}/{}",
            config.redis.password, config.redis.host, config.redis.port, config.redis.database,
        )
    };

    let client = Client::open(redis_url)?;

    ConnectionManager::new(client).await
}
