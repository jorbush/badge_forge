#[cfg(test)]
mod endpoints_tests {
    use crate::utils::test_utils::setup_test_client;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_health_check() {
        let client = setup_test_client().await;
        let response = client.get("/health").await;
        println!("{:?}", response);
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.text().await;
        assert!(body.contains("ok"));
    }

    #[tokio::test]
    async fn test_version_check() {
        let client = setup_test_client().await;
        let response = client.get("/version").await;
        println!("{:?}", response);
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.text().await;
        assert!(body.contains(env!("CARGO_PKG_VERSION")));
    }
}
