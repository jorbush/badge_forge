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
    service::notifier::Notifier,
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
        }
        Ok(())
    }

    async fn get_user_recipes(&self, user_id: &ObjectId) -> Result<Vec<Recipe>, String> {
        let recipes = self.recipes.lock().unwrap();
        Ok(recipes.get(user_id).cloned().unwrap_or_default())
    }

    async fn add_badge_to_user(
        &self,
        user_id: &ObjectId,
        badge: &str,
    ) -> Result<Option<bool>, String> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(user_id) {
            user.ensure_badges();
            if user.badges.contains(&badge.to_string()) {
                Ok(Some(false))
            } else {
                user.badges.push(badge.to_string());
                Ok(Some(true))
            }
        } else {
            Ok(None)
        }
    }
}

pub struct MockNotifier {
    pub notifications: Mutex<Vec<(String, String, serde_json::Value)>>,
}

impl MockNotifier {
    pub fn new() -> Self {
        Self {
            notifications: Mutex::new(Vec::new()),
        }
    }
}

impl Default for MockNotifier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Notifier for MockNotifier {
    async fn send_notification(
        &self,
        notification_type: &str,
        recipient: &str,
        metadata: serde_json::Value,
    ) {
        let mut notes = self.notifications.lock().unwrap();
        notes.push((
            notification_type.to_string(),
            recipient.to_string(),
            metadata,
        ));
    }
}

pub async fn setup_test_client_with_db() -> (TestClient, Arc<MockDatabase>, Arc<MockNotifier>) {
    dotenv().ok();
    // Set up the badge update queue
    let (queue, _) = InMemoryQueue::new(100);
    let queue_arc = Arc::new(queue);
    let badge_queue = queue_arc.clone() as Arc<dyn BadgeUpdateQueue>;

    // Use MockDatabase
    let mock_db = Arc::new(MockDatabase::default());

    // Use MockNotifier
    let mock_notifier = Arc::new(MockNotifier::new());

    // Create the application state
    let state = Arc::new(AppState {
        badge_queue,
        db: mock_db.clone() as Arc<dyn Database>,
        notifier: mock_notifier.clone() as Arc<dyn Notifier>,
    });
    (
        TestClient::new(create_router(state)),
        mock_db,
        mock_notifier,
    )
}

pub async fn setup_test_client() -> TestClient {
    let (client, _, _) = setup_test_client_with_db().await;
    client
}
