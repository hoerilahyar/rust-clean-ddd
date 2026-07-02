use std::sync::Arc;

use redis::aio::ConnectionManager;
use sqlx::MySqlPool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,

    pub db: MySqlPool,

    pub redis: ConnectionManager,
}
