use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "numLikes")]
    pub num_likes: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
