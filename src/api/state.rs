use std::sync::Arc;

use crate::queue::BadgeUpdateQueue;

pub struct AppState {
    pub badge_queue: Arc<dyn BadgeUpdateQueue>,
    pub db_client: mongodb::Client,
    pub db_name: String,
}
