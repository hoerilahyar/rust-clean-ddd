use crate::infrastructure::security::TokenPair;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: TokenPair,

    pub user: LoginUser,
}

#[derive(Debug, Serialize)]
pub struct LoginUser {
    pub id: u64,
    pub username: String,
    pub fullname: String,
    pub email: String,

    pub roles: Vec<String>,

    pub permissions: Vec<String>,
}
