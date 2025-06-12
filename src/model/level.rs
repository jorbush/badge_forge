use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LevelRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(default = "generate_uuid")]
    pub request_id: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}
