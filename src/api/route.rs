use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use std::sync::Arc;

use crate::api::{
    handler::{health_handler, queue_status_handler, update_badges_handler, version_handler},
    state::AppState,
};
use crate::middleware::auth::require_api_key;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/update", post(update_badges_handler))
        .route("/status", get(queue_status_handler))
        .route_layer(from_fn(require_api_key))
        .route("/health", get(health_handler))
        .route("/version", get(version_handler))
        .with_state(state)
}
