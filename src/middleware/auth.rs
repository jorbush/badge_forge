use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::env;

/// Compares two byte slices in constant time to prevent timing attacks.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

pub async fn require_api_key(request: Request, next: Next) -> Response {
    let api_key = match env::var("API_KEY") {
        Ok(val) if !val.trim().is_empty() => val,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "API_KEY is not configured on the server",
            )
                .into_response();
        }
    };

    if let Some(auth_header) = request.headers().get("X-API-Key")
        && let Ok(header_value) = auth_header.to_str()
        && constant_time_eq(header_value.as_bytes(), api_key.as_bytes())
    {
        return next.run(request).await;
    }

    (StatusCode::UNAUTHORIZED, "Invalid or missing API key").into_response()
}

#[cfg(test)]
mod tests {
    use super::constant_time_eq;

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"secret-123", b"secret-123"));
        assert!(!constant_time_eq(b"secret-123", b"secret-124"));
        assert!(!constant_time_eq(b"short", b"longer-secret"));
        assert!(constant_time_eq(b"", b""));
    }
}
