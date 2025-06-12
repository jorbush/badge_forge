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
    // Create the application state
    let state = Arc::new(AppState { badge_queue });
    TestClient::new(create_router(state))
}
