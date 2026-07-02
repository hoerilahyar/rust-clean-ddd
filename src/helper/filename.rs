use chrono::Utc;
use uuid::Uuid;

pub fn generate(extension: &str) -> String {
    format!(
        "{}_{}.{}",
        Utc::now().timestamp(),
        Uuid::new_v4(),
        extension
    )
}
