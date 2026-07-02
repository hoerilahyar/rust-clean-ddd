use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: u64,
    pub username: String,
    pub roles: Vec<String>,
    pub iss: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: u64,
    pub device_id: String,

    pub iss: String,
    pub iat: usize,
    pub exp: usize,
}
