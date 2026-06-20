use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopRecipeRequest {
    pub category: String,
    pub user_id: String,
    pub recipe_id: Option<String>,
}
