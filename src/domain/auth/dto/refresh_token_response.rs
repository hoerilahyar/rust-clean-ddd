use crate::infrastructure::security::TokenPair;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    pub token: TokenPair,
}
