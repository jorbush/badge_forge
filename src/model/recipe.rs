use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: String,
    pub title: String,
    pub userId: String,
    pub numLikes: i32,
}
