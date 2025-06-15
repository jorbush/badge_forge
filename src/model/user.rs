use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub name: Option<String>,
    pub email: Option<String>,
    pub level: i32,
    #[serde(default)]
    pub badges: Vec<String>,
}

impl User {
    pub fn ensure_badges(&mut self) {
        if self.badges.is_empty() {
            self.badges = Vec::new();
        }
    }
}
