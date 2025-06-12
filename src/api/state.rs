use std::sync::Arc;

use crate::queue::BadgeUpdateQueue;

pub struct AppState {
    pub badge_queue: Arc<dyn BadgeUpdateQueue>,
}
