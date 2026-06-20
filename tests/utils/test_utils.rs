use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use axum::test_helpers::TestClient;
use badge_forge::{
    api::{route::create_router, state::AppState},
    model::recipe::Recipe,
    model::user::User,
    queue::{BadgeUpdateQueue, InMemoryQueue},
    service::db::Database,
};
use dotenv::dotenv;
use mongodb::bson::oid::ObjectId;

pub struct MockDatabase {
    pub users: Mutex<HashMap<ObjectId, User>>,
    pub recipes: Mutex<HashMap<ObjectId, Vec<Recipe>>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MockDatabase {
    fn default() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            recipes: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Database for MockDatabase {
    async fn find_user(&self, user_id: &ObjectId) -> Result<Option<User>, String> {
        let users = self.users.lock().unwrap();
        Ok(users.get(user_id).cloned())
    }

    async fn update_user_badges(
        &self,
        user_id: &ObjectId,
        badges: &[String],
    ) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(user_id) {
            user.badges = badges.to_vec();
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    async fn update_user_badges_and_level(
        &self,
        user_id: &ObjectId,
        badges: &[String],
        level: i32,
        verified: bool,
    ) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(user_id) {
            user.badges = badges.to_vec();
            user.level = level;
            user.verified = Some(verified);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    async fn get_user_recipes(&self, user_id: &ObjectId) -> Result<Vec<Recipe>, String> {
        let recipes = self.recipes.lock().unwrap();
        Ok(recipes.get(user_id).cloned().unwrap_or_default())
    }
}

pub async fn setup_test_client_with_db() -> (TestClient, Arc<MockDatabase>) {
    dotenv().ok();
    // Set up the badge update queue
    let (queue, _) = InMemoryQueue::new(100);
    let queue_arc = Arc::new(queue);
    let badge_queue = queue_arc.clone() as Arc<dyn BadgeUpdateQueue>;

    // Use MockDatabase
    let mock_db = Arc::new(MockDatabase::default());

    // Create the application state
    let state = Arc::new(AppState {
        badge_queue,
        db: mock_db.clone() as Arc<dyn Database>,
    });
    (TestClient::new(create_router(state)), mock_db)
}

pub async fn setup_test_client() -> TestClient {
    let (client, _) = setup_test_client_with_db().await;
    client
}
