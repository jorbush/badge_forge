use axum::{Json, response::IntoResponse};

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
