use crate::infrastructure::security::TokenPair;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub token: TokenPair,
}
