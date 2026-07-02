mod api;
mod middleware;
mod model;
mod queue;
mod service;
mod utils;
use crate::queue::BadgeUpdateQueue;
use api::route::create_router;
use api::state::AppState;
use dotenv::dotenv;
use mongodb::{Client, options::ClientOptions};
use queue::InMemoryQueue;
use service::badge_processor::BadgeForgeProcessor;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    utils::logging::init_logging("badge_forge");

    info!("Starting Badge Forge API");

    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| "badgeforge".to_string());

    let client_options = ClientOptions::parse(mongodb_uri).await?;
    let db_client = Client::with_options(client_options)?;

    // Ping the database to verify connection
    db_client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 })
        .await?;
    info!("Connected to MongoDB");

    let (queue, receiver) = InMemoryQueue::new(100);
    let queue_arc = Arc::new(queue);
    let badge_queue = queue_arc.clone() as Arc<dyn BadgeUpdateQueue>;

    let db = Arc::new(service::db::MongoDatabase::new(
        db_client.clone(),
        db_name.clone(),
    )) as Arc<dyn service::db::Database>;

    let notifier = Arc::new(service::notifier::HttpNotifier::from_env())
        as Arc<dyn service::notifier::Notifier>;

    let processor = BadgeForgeProcessor::new(db.clone(), notifier.clone());
    processor.start(receiver, queue_arc.clone()).await;

    let state = Arc::new(AppState {
        badge_queue,
        db,
        notifier,
    });
    let app = create_router(state);
    info!("Badge Forge API started successfully on port 4000 🎖️");
    axum::serve(tokio::net::TcpListener::bind("0.0.0.0:4000").await?, app).await?;
    Ok(())
}
