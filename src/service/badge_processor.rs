use std::sync::Arc;

use mongodb::bson::oid::ObjectId;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::{
    model::level::LevelRequest,
    queue::InMemoryQueue,
    service::{db::Database, notifier::Notifier},
    utils::{badge::assign_badges, level::calculate_level},
};

pub struct BadgeForgeProcessor {
    db: Arc<dyn Database>,
    notifier: Arc<dyn Notifier>,
}

impl BadgeForgeProcessor {
    pub fn new(db: Arc<dyn Database>, notifier: Arc<dyn Notifier>) -> Self {
        Self { db, notifier }
    }

    pub async fn start(
        self,
        mut receiver: mpsc::Receiver<LevelRequest>,
        queue: Arc<InMemoryQueue>,
    ) {
        tokio::spawn(async move {
            info!("Badge Forge Processor started");
            while let Some(request) = receiver.recv().await {
                let request_id = request.request_id.clone();
                if let Err(e) = self.process_request(request).await {
                    error!("Error processing badge update request: {}", e);
                }
                queue.remove_request(&request_id).await;
            }
        });
    }

    async fn process_request(&self, request: LevelRequest) -> Result<(), String> {
        info!("Processing badge update for user: {}", request.user_id);

        let user_id = match ObjectId::parse_str(&request.user_id) {
            Ok(id) => id,
            Err(_) => return Err(format!("Invalid user ID format: {}", request.user_id)),
        };

        let mut user = self
            .db
            .find_user(&user_id)
            .await?
            .ok_or_else(|| format!("User not found: {}", request.user_id))?;
        user.ensure_badges();

        let user_recipes = self.db.get_user_recipes(&user_id).await?;

        let num_recipes = user_recipes.len();
        let user_num_likes: u32 = user_recipes.iter().map(|r| r.num_likes as u32).sum();

        let new_user_level = calculate_level(num_recipes as u32, user_num_likes) as i32;

        let mut updated_badges = user.badges.clone();
        assign_badges(&mut updated_badges, new_user_level, user_recipes);

        let is_already_verified = user.verified.unwrap_or(false);
        let verified = if num_recipes >= 30 {
            true
        } else {
            is_already_verified
        };

        let newly_verified = verified && !is_already_verified;

        self.db
            .update_user_badges_and_level(&user_id, &updated_badges, new_user_level, verified)
            .await?;

        let old_badges: std::collections::HashSet<_> = user.badges.iter().collect();
        let new_badges: Vec<_> = updated_badges
            .iter()
            .filter(|b| !old_badges.contains(b))
            .collect();

        if !new_badges.is_empty()
            && let Some(ref email) = user.email
        {
            for badge in new_badges {
                let metadata = serde_json::json!({
                    "badgeName": badge,
                    "userId": &request.user_id
                });

                self.notifier
                    .send_notification("NEW_BADGE", email, metadata)
                    .await;
            }
        }

        if newly_verified && let Some(ref email) = user.email {
            let metadata = serde_json::json!({
                "userId": &request.user_id
            });

            self.notifier
                .send_notification("VERIFIED", email, metadata)
                .await;
        }

        info!(
            "Updated level and badges for user {}: level {}, badges {:?}, verified {}",
            request.user_id, new_user_level, updated_badges, verified
        );

        Ok(())
    }
}
