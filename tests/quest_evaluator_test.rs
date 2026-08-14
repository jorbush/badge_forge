mod utils;

use axum::http::StatusCode;
use badge_forge::model::user::User;
use mongodb::bson::oid::ObjectId;
use serde_json::json;
use utils::test_utils::setup_test_client_with_db;

fn get_test_api_key() -> String {
    dotenv::dotenv().ok();
    std::env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string())
}

#[tokio::test]
async fn test_evaluate_quest_unauthorized() {
    let (client, _, _) = setup_test_client_with_db().await;
    let response = client
        .post("/api/evaluate")
        .json(&json!({
            "userId": "507f1f77bcf86cd799439011",
            "event": "QUEST_FULFILLED"
        }))
        .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_evaluate_quest_invalid_event() {
    let (client, _, _) = setup_test_client_with_db().await;
    let response = client
        .post("/api/evaluate")
        .header("X-API-Key", get_test_api_key())
        .json(&json!({
            "userId": "507f1f77bcf86cd799439011",
            "event": "INVALID_EVENT"
        }))
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_evaluate_quest_threshold_1() {
    let (client, db, notifier) = setup_test_client_with_db().await;
    let user_oid = ObjectId::new();
    let email = "solver1@example.com".to_string();

    let dummy_user = User {
        _id: user_oid,
        name: Some("Solver One".to_string()),
        email: Some(email.clone()),
        level: 1,
        badges: vec![],
        verified: Some(false),
    };

    {
        let mut users = db.users.lock().unwrap();
        users.insert(user_oid, dummy_user);

        let mut quests = db.completed_quests.lock().unwrap();
        quests.insert(user_oid, 1);
    }

    let response = client
        .post("/api/evaluate")
        .header("X-API-Key", get_test_api_key())
        .json(&json!({
            "userId": user_oid.to_hex(),
            "event": "QUEST_FULFILLED"
        }))
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_str(&response.text().await).unwrap();
    assert_eq!(body["status"], "success");
    let granted = body["granted_badges"].as_array().unwrap();
    assert_eq!(granted.len(), 1);
    assert_eq!(granted[0], "quest_solver_1.webp");

    // Verify user badges in db
    {
        let users = db.users.lock().unwrap();
        let user = users.get(&user_oid).unwrap();
        assert!(user.badges.contains(&"quest_solver_1.webp".to_string()));
    }

    // Verify notification was sent
    {
        let notes = notifier.notifications.lock().unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].0, "NEW_BADGE");
        assert_eq!(notes[0].1, email);
    }
}

#[tokio::test]
async fn test_evaluate_quest_threshold_10() {
    let (client, db, _) = setup_test_client_with_db().await;
    let user_oid = ObjectId::new();

    let dummy_user = User {
        _id: user_oid,
        name: Some("Solver Ten".to_string()),
        email: Some("solver10@example.com".to_string()),
        level: 1,
        badges: vec![],
        verified: Some(false),
    };

    {
        let mut users = db.users.lock().unwrap();
        users.insert(user_oid, dummy_user);

        let mut quests = db.completed_quests.lock().unwrap();
        quests.insert(user_oid, 10);
    }

    let response = client
        .post("/api/evaluate")
        .header("X-API-Key", get_test_api_key())
        .json(&json!({
            "userId": user_oid.to_hex(),
            "event": "QUEST_FULFILLED"
        }))
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_str(&response.text().await).unwrap();
    let granted = body["granted_badges"].as_array().unwrap();
    assert_eq!(granted.len(), 2);
    assert!(granted.contains(&json!("quest_solver_1.webp")));
    assert!(granted.contains(&json!("quest_solver_10.webp")));
}

#[tokio::test]
async fn test_evaluate_quest_threshold_25() {
    let (client, db, _) = setup_test_client_with_db().await;
    let user_oid = ObjectId::new();

    let dummy_user = User {
        _id: user_oid,
        name: Some("Solver Master".to_string()),
        email: Some("master@example.com".to_string()),
        level: 1,
        badges: vec![],
        verified: Some(false),
    };

    {
        let mut users = db.users.lock().unwrap();
        users.insert(user_oid, dummy_user);

        let mut quests = db.completed_quests.lock().unwrap();
        quests.insert(user_oid, 25);
    }

    let response = client
        .post("/api/evaluate")
        .header("X-API-Key", get_test_api_key())
        .json(&json!({
            "userId": user_oid.to_hex(),
            "event": "QUEST_FULFILLED"
        }))
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_str(&response.text().await).unwrap();
    let granted = body["granted_badges"].as_array().unwrap();
    assert_eq!(granted.len(), 3);
    assert!(granted.contains(&json!("quest_solver_1.webp")));
    assert!(granted.contains(&json!("quest_solver_10.webp")));
    assert!(granted.contains(&json!("quest_solver_25.webp")));
}
