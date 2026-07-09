use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Raw row as stored in MySQL. `permissions` comes back as a JSON string and
/// is parsed into `Vec<String>` on the domain struct (same workaround
/// pattern used by `SystemSettingRow` for the ENUM column).
#[derive(Debug, Clone, FromRow)]
pub struct ApiKeyRow {
    pub id: u64,
    pub name: String,
    pub key_prefix: String,
    pub key_hash: String,
    pub permissions: serde_json::Value,
    pub is_active: bool,
    pub expires_at: Option<NaiveDateTime>,
    pub last_used_at: Option<NaiveDateTime>,
    pub created_by: Option<u64>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: u64,
    pub name: String,
    pub key_prefix: String,
    pub key_hash: String,
    pub permissions: Vec<String>,
    pub is_active: bool,
    pub expires_at: Option<NaiveDateTime>,
    pub last_used_at: Option<NaiveDateTime>,
    pub created_by: Option<u64>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl ApiKey {
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => expires_at <= chrono::Utc::now().naive_utc(),
            None => false,
        }
    }

    pub fn is_usable(&self) -> bool {
        self.is_active && !self.is_expired()
    }
}

impl TryFrom<ApiKeyRow> for ApiKey {
    type Error = anyhow::Error;

    fn try_from(row: ApiKeyRow) -> Result<Self, Self::Error> {
        let permissions: Vec<String> = serde_json::from_value(row.permissions)?;

        Ok(ApiKey {
            id: row.id,
            name: row.name,
            key_prefix: row.key_prefix,
            key_hash: row.key_hash,
            permissions,
            is_active: row.is_active,
            expires_at: row.expires_at,
            last_used_at: row.last_used_at,
            created_by: row.created_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        })
    }
}
