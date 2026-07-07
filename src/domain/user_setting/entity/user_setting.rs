use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingDataType {
    String,
    Number,
    Boolean,
    Json,
}

impl SettingDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SettingDataType::String => "string",
            SettingDataType::Number => "number",
            SettingDataType::Boolean => "boolean",
            SettingDataType::Json => "json",
        }
    }

    pub fn parse(value: &str) -> anyhow::Result<Self> {
        match value {
            "string" => Ok(SettingDataType::String),
            "number" => Ok(SettingDataType::Number),
            "boolean" => Ok(SettingDataType::Boolean),
            "json" => Ok(SettingDataType::Json),
            other => anyhow::bail!("Unknown data_type: {other}"),
        }
    }
}

impl std::fmt::Display for SettingDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// Row mentah persis kolom di database — data_type dibaca sebagai String
#[derive(Debug, Clone, FromRow)]
pub struct UserSettingRow {
    pub id: u64,
    pub user_id: u64,
    pub setting_key: String,
    pub setting_value: Option<String>,
    pub data_type: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

// Entity domain yang dipakai di service/repository trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSetting {
    pub id: u64,
    pub user_id: u64,
    pub setting_key: String,
    pub setting_value: Option<String>,
    pub data_type: SettingDataType,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl TryFrom<UserSettingRow> for UserSetting {
    type Error = anyhow::Error;

    fn try_from(row: UserSettingRow) -> Result<Self, Self::Error> {
        Ok(UserSetting {
            id: row.id,
            user_id: row.user_id,
            setting_key: row.setting_key,
            setting_value: row.setting_value,
            data_type: SettingDataType::parse(&row.data_type)?,
            description: row.description,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        })
    }
}
