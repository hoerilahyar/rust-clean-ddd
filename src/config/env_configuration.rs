use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct EnvConfig {
    pub mysql_host: String,
    pub mysql_port: u16,
    pub mysql_database: String,
    pub mysql_username: String,
    pub mysql_password: String,

    pub jwt_secret: String,
}
