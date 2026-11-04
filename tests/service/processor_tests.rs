#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mongodb::bson::oid::ObjectId;
    use std::sync::Arc;

    use crate::utils::test_utils::{MockDatabase, MockNotifier};
    use badge_forge::model::level::LevelRequest;
    use badge_forge::model::recipe::Recipe;
    use badge_forge::model::user::User;
    use badge_forge::service::badge_processor::BadgeForgeProcessor;

    #[tokio::test]
    async fn test_processor_awards_remix_karma_and_updates_level() {
        let mock_db = Arc::new(MockDatabase::new());
        let mock_notifier = Arc::new(MockNotifier::new());

        let user_id = ObjectId::new();
        let user = User {
            _id: user_id,
            name: Some("Chef Author".to_string()),
            email: Some("author@test.com".to_string()),
            badges: vec![],
            level: 0,
            verified: Some(false),
        };
        mock_db.users.lock().unwrap().insert(user_id, user);

        // Add 2 recipes with 5 likes total
        let recipe1 = Recipe {
            _id: ObjectId::new(),
            user_id,
            num_likes: 2,
            created_at: Utc::now(),
        };
        let recipe2 = Recipe {
            _id: ObjectId::new(),
            user_id,
            num_likes: 3,
            created_at: Utc::now(),
        };
        mock_db
            .recipes
            .lock()
            .unwrap()
            .insert(user_id, vec![recipe1, recipe2]);

        // Add 4 community remixes of the author's recipes
        mock_db.remixes.lock().unwrap().insert(user_id, 4);

        let processor = BadgeForgeProcessor::new(mock_db.clone(), mock_notifier.clone());

        let request = LevelRequest {
            user_id: user_id.to_hex(),
            request_id: "req-123".to_string(),
            created_at: Utc::now(),
        };

        let result = processor.process_request(request).await;
        assert!(result.is_ok());

        // Expected level: 2 recipes + 5 likes + 4 remixes = 11
        let updated_user = mock_db
            .users
            .lock()
            .unwrap()
            .get(&user_id)
            .cloned()
            .unwrap();
        assert_eq!(updated_user.level, 11);
    }
}
