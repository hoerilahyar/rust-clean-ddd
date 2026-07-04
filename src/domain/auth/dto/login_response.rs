use serde::Serialize;
use utoipa::ToSchema;

use crate::infrastructure::security::TokenPair;

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: TokenPair,

    pub user: LoginUser,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginUser {
    pub id: u64,
    pub username: String,
    pub fullname: String,
    pub email: String,

    pub roles: Vec<String>,

    pub permissions: Vec<String>,
}
