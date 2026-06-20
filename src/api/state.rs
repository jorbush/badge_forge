use std::sync::Arc;

use crate::queue::BadgeUpdateQueue;

use crate::service::db::Database;

pub struct AppState {
    pub badge_queue: Arc<dyn BadgeUpdateQueue>,
    pub db: Arc<dyn Database>,
}
