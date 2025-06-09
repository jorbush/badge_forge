use axum::test_helpers::TestClient;
use badge_forge::api::route::create_router;

pub async fn setup_test_client() -> TestClient {
    TestClient::new(create_router())
}
