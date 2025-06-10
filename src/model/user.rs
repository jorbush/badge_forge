use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub level: i32,
    pub badges: Vec<String>,
}
