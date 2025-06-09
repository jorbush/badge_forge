use axum::{Router, routing::get};

use crate::api::handler::{health_handler, version_handler};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/version", get(version_handler))
}
