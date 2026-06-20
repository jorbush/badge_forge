use crate::model::recipe::Recipe;
use crate::model::user::User;
use async_trait::async_trait;
use futures_util::stream::TryStreamExt;
use mongodb::Client;
use mongodb::bson::oid::ObjectId;

#[async_trait]
pub trait Database: Send + Sync {
    async fn find_user(&self, user_id: &ObjectId) -> Result<Option<User>, String>;
    async fn update_user_badges(&self, user_id: &ObjectId, badges: &[String])
    -> Result<(), String>;
    async fn update_user_badges_and_level(
        &self,
        user_id: &ObjectId,
        badges: &[String],
        level: i32,
        verified: bool,
    ) -> Result<(), String>;
    async fn get_user_recipes(&self, user_id: &ObjectId) -> Result<Vec<Recipe>, String>;
}

pub struct MongoDatabase {
    client: Client,
    db_name: String,
}

impl MongoDatabase {
    pub fn new(client: Client, db_name: String) -> Self {
        Self { client, db_name }
    }
}

#[async_trait]
impl Database for MongoDatabase {
    async fn find_user(&self, user_id: &ObjectId) -> Result<Option<User>, String> {
        let user_collection = self
            .client
            .database(&self.db_name)
            .collection::<User>("User");
        user_collection
            .find_one(mongodb::bson::doc! { "_id": user_id })
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    async fn update_user_badges(
        &self,
        user_id: &ObjectId,
        badges: &[String],
    ) -> Result<(), String> {
        let user_collection = self
            .client
            .database(&self.db_name)
            .collection::<User>("User");
        user_collection
            .update_one(
                mongodb::bson::doc! { "_id": user_id },
                mongodb::bson::doc! { "$set": { "badges": badges } },
            )
            .await
            .map(|_| ())
            .map_err(|e| format!("Database error: {}", e))
    }

    async fn update_user_badges_and_level(
        &self,
        user_id: &ObjectId,
        badges: &[String],
        level: i32,
        verified: bool,
    ) -> Result<(), String> {
        let user_collection = self
            .client
            .database(&self.db_name)
            .collection::<User>("User");
        user_collection
            .update_one(
                mongodb::bson::doc! { "_id": user_id },
                mongodb::bson::doc! { "$set": { "badges": badges, "level": level, "verified": verified } },
            )
            .await
            .map(|_| ())
            .map_err(|e| format!("Database error: {}", e))
    }

    async fn get_user_recipes(&self, user_id: &ObjectId) -> Result<Vec<Recipe>, String> {
        let recipe_collection = self
            .client
            .database(&self.db_name)
            .collection::<Recipe>("Recipe");
        let mut cursor = recipe_collection
            .find(mongodb::bson::doc! { "userId": user_id })
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let mut recipes = Vec::new();
        while let Some(recipe) = cursor
            .try_next()
            .await
            .map_err(|e| format!("Database error: {}", e))?
        {
            recipes.push(recipe);
        }
        Ok(recipes)
    }
}
