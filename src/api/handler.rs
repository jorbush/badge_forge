use crate::api::state::AppState;
use crate::model::level::LevelRequest;
use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;

pub async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok"
    }))
}

pub async fn version_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub async fn update_badges_handler(
    State(state): State<Arc<AppState>>,
    Json(mut request): Json<LevelRequest>,
) -> impl IntoResponse {
    if request.request_id.is_empty() {
        request.request_id = uuid::Uuid::new_v4().to_string();
    }

    if request.created_at.timestamp() == 0 {
        request.created_at = chrono::Utc::now();
    }
    match state.badge_queue.enqueue(request.clone()).await {
        Ok(_) => Json(json!({
            "status": "queued",
            "message": "Badge update request has been queued for processing",
            "user_id": request.user_id
        }))
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Failed to queue badge update request: {}", e)
            })),
        )
            .into_response(),
    }
}

pub async fn queue_status_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pending_requests = state.badge_queue.get_pending_requests().await;
    let pending_count = pending_requests.len();

    Json(json!({
        "status": "ok",
        "pending_count": pending_count,
        "pending_requests": pending_requests
    }))
}
