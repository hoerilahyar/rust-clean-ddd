use crate::config::Config;
use redis::{Client, aio::ConnectionManager};

pub async fn connect(config: &Config) -> Result<ConnectionManager, redis::RedisError> {
    let redis_url = if config.redis.password.is_empty() {
        format!("redis://{}:{}/0", config.redis.host, config.redis.port)
    } else {
        format!(
            "redis://:{}@{}:{}/0",
            config.redis.password, config.redis.host, config.redis.port
        )
    };

    let client = Client::open(redis_url)?;
    ConnectionManager::new(client).await
}
