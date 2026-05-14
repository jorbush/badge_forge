#[cfg(test)]
mod tests {
    use axum::{Router, routing::post};
    use badge_forge::service::notifier::Notifier;
    use serde_json::json;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::net::TcpListener;

    async fn run_mock_server() -> (String, Arc<AtomicUsize>) {
        let req_count = Arc::new(AtomicUsize::new(0));
        let req_count_clone = req_count.clone();

        let app = Router::new().route(
            "/notifications",
            post(move || {
                req_count_clone.fetch_add(1, Ordering::SeqCst);
                async { "OK" }
            }),
        );

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}", addr);

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        (url, req_count)
    }

    #[tokio::test]
    async fn test_send_notification_success() {
        let (url, req_count) = run_mock_server().await;

        let notifier = Notifier::new(url, "test_api_key".to_string());
        let metadata = json!({ "userId": "123" });

        notifier
            .send_notification("VERIFIED", "user@example.com", metadata)
            .await;

        // The mock server should have received 1 request
        assert_eq!(req_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_send_notification_empty_url() {
        let notifier = Notifier::new("".to_string(), "test_api_key".to_string());
        let metadata = json!({ "userId": "123" });

        // Should just log and return without error (or panic)
        notifier
            .send_notification("VERIFIED", "user@example.com", metadata)
            .await;
    }
}
