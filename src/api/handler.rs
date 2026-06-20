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

pub async fn award_top_recipe_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<crate::model::top_recipe_request::TopRecipeRequest>,
) -> impl IntoResponse {
    tracing::info!("Award top recipe request: {:?}", request);

    let category = match crate::model::category::Category::parse(&request.category) {
        Some(cat) => cat,
        None => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": format!("Invalid category: {}", request.category)
                })),
            )
                .into_response();
        }
    };
    let badge_name = category.badge_name();

    let user_id = match mongodb::bson::oid::ObjectId::parse_str(&request.user_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": format!("Invalid user ID format: {}", request.user_id)
                })),
            )
                .into_response();
        }
    };

    let user = match state.db.find_user(&user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "error",
                    "message": format!("User not found: {}", request.user_id)
                })),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch user in award_top_recipe_handler: {}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Database error: {}", e)
                })),
            )
                .into_response();
        }
    };

    match state.db.add_badge_to_user(&user_id, badge_name).await {
        Ok(Some(true)) => {
            tracing::info!(
                "Successfully awarded badge {} to user {}",
                badge_name,
                request.user_id
            );

            // Send notification using state.notifier
            if let Some(ref email) = user.email {
                let metadata = serde_json::json!({
                    "badgeName": badge_name,
                    "userId": &request.user_id
                });
                state
                    .notifier
                    .send_notification("NEW_BADGE", email, metadata)
                    .await;
            }

            Json(json!({
                "status": "success",
                "message": format!("Badge {} awarded successfully", badge_name),
                "badge": badge_name
            }))
            .into_response()
        }
        Ok(Some(false)) => {
            tracing::info!(
                "User {} already has badge {}. Skipping award.",
                request.user_id,
                badge_name
            );
            Json(json!({
                "status": "already_awarded",
                "message": "User already has this badge",
                "badge": badge_name
            }))
            .into_response()
        }
        Ok(None) => (
            axum::http::StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": format!("User not found: {}", request.user_id)
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!(
                "Failed to update user badges in award_top_recipe_handler: {}",
                e
            );
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to update user badges: {}", e)
                })),
            )
                .into_response()
        }
    }
}
