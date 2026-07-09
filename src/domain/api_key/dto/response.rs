use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Safe-to-return representation of an API key. Never includes the secret
/// or its hash.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyResponse {
    pub id: u64,
    pub name: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub is_active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_by: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ApiKeyListResponse {
    pub items: Vec<ApiKeyResponse>,
}

/// Returned only once, immediately after creation. The plaintext `api_key`
/// cannot be retrieved again afterwards — only its hash is persisted.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CreateApiKeyResponse {
    pub id: u64,
    pub name: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub is_active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Option<u64>,
    pub created_at: DateTime<Utc>,
    /// Full plaintext key. Store this now — it will never be shown again.
    pub api_key: String,
}
