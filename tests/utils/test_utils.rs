use std::sync::Arc;

use axum::test_helpers::TestClient;
use badge_forge::{
    api::{route::create_router, state::AppState},
    queue::{BadgeUpdateQueue, InMemoryQueue},
};
use dotenv::dotenv;

pub async fn setup_test_client() -> TestClient {
    dotenv().ok();
    // Set up the badge update queue
    let (queue, _) = InMemoryQueue::new(100);
    let queue_arc = Arc::new(queue);
    let badge_queue = queue_arc.clone() as Arc<dyn BadgeUpdateQueue>;

    // Set up MongoDB connection
    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| "badgeforge".to_string());

    let client_options = mongodb::options::ClientOptions::parse(mongodb_uri)
        .await
        .unwrap();
    let db_client = mongodb::Client::with_options(client_options).unwrap();

    // Create the application state
    let state = Arc::new(AppState {
        badge_queue,
        db_client,
        db_name,
    });
    TestClient::new(create_router(state))
}
