use std::sync::Arc;

use crate::queue::BadgeUpdateQueue;
use crate::service::db::Database;
use crate::service::notifier::Notifier;

pub struct AppState {
    pub badge_queue: Arc<dyn BadgeUpdateQueue>,
    pub db: Arc<dyn Database>,
    pub notifier: Arc<dyn Notifier>,
}
