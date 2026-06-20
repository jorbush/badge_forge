#[cfg(test)]
mod endpoints_tests {
    use crate::utils::test_utils::{setup_test_client, setup_test_client_with_db};
    use axum::http::StatusCode;
    use badge_forge::model::user::User;
    use mongodb::bson::oid::ObjectId;
    use serde_json::json;

    fn get_test_api_key() -> String {
        dotenv::dotenv().ok();
        std::env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string())
    }

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

    #[tokio::test]
    async fn test_award_top_recipe_unauthorized() {
        let client = setup_test_client().await;
        let response = client
            .post("/award-top-recipe")
            .json(&json!({
                "category": "week",
                "user_id": "507f1f77bcf86cd799439011"
            }))
            .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_award_top_recipe_invalid_category() {
        let client = setup_test_client().await;
        let response = client
            .post("/award-top-recipe")
            .header("X-API-Key", get_test_api_key())
            .json(&json!({
                "category": "invalid_category",
                "user_id": "507f1f77bcf86cd799439011"
            }))
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: serde_json::Value = serde_json::from_str(&response.text().await).unwrap();
        assert_eq!(body["status"], "error");
        assert!(
            body["message"]
                .as_str()
                .unwrap()
                .contains("Invalid category")
        );
    }

    #[tokio::test]
    async fn test_award_top_recipe_invalid_user_id() {
        let client = setup_test_client().await;
        let response = client
            .post("/award-top-recipe")
            .header("X-API-Key", get_test_api_key())
            .json(&json!({
                "category": "week",
                "user_id": "invalid_object_id"
            }))
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: serde_json::Value = serde_json::from_str(&response.text().await).unwrap();
        assert_eq!(body["status"], "error");
        assert!(
            body["message"]
                .as_str()
                .unwrap()
                .contains("Invalid user ID format")
        );
    }

    #[tokio::test]
    async fn test_award_top_recipe_user_not_found() {
        let client = setup_test_client().await;
        let non_existent_id = ObjectId::new().to_hex();
        let response = client
            .post("/award-top-recipe")
            .header("X-API-Key", get_test_api_key())
            .json(&json!({
                "category": "week",
                "user_id": non_existent_id
            }))
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: serde_json::Value = serde_json::from_str(&response.text().await).unwrap();
        assert_eq!(body["status"], "error");
        assert!(body["message"].as_str().unwrap().contains("User not found"));
    }

    #[tokio::test]
    async fn test_award_top_recipe_success() {
        let (client, db, notifier) = setup_test_client_with_db().await;

        // Insert a dummy user directly into MockDatabase
        let user_oid = ObjectId::new();
        let email = format!("test_winner_{}@example.com", user_oid.to_hex());
        let dummy_user = User {
            _id: user_oid,
            name: Some("Test User".to_string()),
            email: Some(email.clone()),
            level: 1,
            badges: vec![],
            verified: Some(false),
        };

        {
            let mut users = db.users.lock().unwrap();
            users.insert(user_oid, dummy_user);
        }

        // Award weekly badge
        let response = client
            .post("/award-top-recipe")
            .header("X-API-Key", get_test_api_key())
            .json(&json!({
                "category": "week",
                "user_id": user_oid.to_hex()
            }))
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_str(&response.text().await).unwrap();
        assert_eq!(body["status"], "success");
        assert_eq!(body["badge"], "recipe_of_the_week");

        // Verify badge added in DB
        {
            let users = db.users.lock().unwrap();
            let user_in_db = users.get(&user_oid).unwrap();
            assert!(
                user_in_db
                    .badges
                    .contains(&"recipe_of_the_week".to_string())
            );
        }

        // Verify notification was sent
        {
            let notes = notifier.notifications.lock().unwrap();
            assert_eq!(notes.len(), 1);
            assert_eq!(notes[0].0, "NEW_BADGE");
            assert_eq!(notes[0].1, email);
        }

        // Call again and verify it detects already_awarded
        let response_dup = client
            .post("/award-top-recipe")
            .header("X-API-Key", get_test_api_key())
            .json(&json!({
                "category": "week",
                "user_id": user_oid.to_hex()
            }))
            .await;

        assert_eq!(response_dup.status(), StatusCode::OK);
        let body_dup: serde_json::Value = serde_json::from_str(&response_dup.text().await).unwrap();
        assert_eq!(body_dup["status"], "already_awarded");

        // Notification count should still be 1 (duplicate doesn't trigger new notification)
        {
            let notes = notifier.notifications.lock().unwrap();
            assert_eq!(notes.len(), 1);
        }
    }
}
