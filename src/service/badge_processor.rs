use std::sync::Arc;

use futures_util::stream::TryStreamExt;
use mongodb::{Client, Collection, bson::oid::ObjectId};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::{
    model::{level::LevelRequest, recipe::Recipe, user::User},
    queue::InMemoryQueue,
    service::notifier::Notifier,
    utils::{badge::assign_badges, level::calculate_level},
};

pub struct BadgeForgeProcessor {
    db_client: Client,
    db_name: String,
    notifier: Notifier,
}

impl BadgeForgeProcessor {
    pub fn new(db_client: Client, db_name: String) -> Self {
        Self {
            db_client,
            db_name,
            notifier: Notifier::from_env(),
        }
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

        let user_collection: Collection<User> =
            self.db_client.database(&self.db_name).collection("User");

        let user_id = match ObjectId::parse_str(&request.user_id) {
            Ok(id) => id,
            Err(_) => return Err(format!("Invalid user ID format: {}", request.user_id)),
        };

        let mut user = user_collection
            .find_one(mongodb::bson::doc! { "_id": &user_id })
            .await
            .map_err(|e| format!("Failed to fetch user: {}", e))?
            .ok_or_else(|| format!("User not found: {}", request.user_id))?;
        user.ensure_badges();

        let recipe_collection: Collection<Recipe> =
            self.db_client.database(&self.db_name).collection("Recipe");
        let mut recipes = recipe_collection
            .find(mongodb::bson::doc! { "userId": user_id })
            .await
            .map_err(|e| format!("Failed to fetch recipes: {}", e))?;

        let mut user_recipes = Vec::new();
        while let Some(recipe) = recipes
            .try_next()
            .await
            .map_err(|e| format!("Error iterating recipes: {}", e))?
        {
            user_recipes.push(recipe);
        }

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

        user_collection
            .update_one(
                mongodb::bson::doc! { "_id": user_id },
                mongodb::bson::doc! { "$set": { "badges": &updated_badges, "level": new_user_level, "verified": verified } },
            )
            .await
            .map_err(|e| format!("Failed to update user badges: {}", e))?;

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
