use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::env;

pub async fn require_api_key(request: Request, next: Next) -> Response {
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string());

    if let Some(auth_header) = request.headers().get("X-API-Key") {
        if let Ok(header_value) = auth_header.to_str() {
            // Check if the API key matches
            if header_value == api_key {
                return next.run(request).await;
            }
        }
    }

    (StatusCode::UNAUTHORIZED, "Invalid or missing API key").into_response()
}
