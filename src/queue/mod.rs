use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tracing::info;

use crate::model::level::LevelRequest;

/// Queue trait defining operations for a badge update queue
#[async_trait]
pub trait BadgeUpdateQueue: Send + Sync {
    async fn enqueue(&self, request: LevelRequest) -> Result<(), String>;
    async fn get_pending_requests(&self) -> Vec<LevelRequest>;
}

pub struct InMemoryQueue {
    sender: mpsc::Sender<LevelRequest>,
    pending_requests: Arc<Mutex<Vec<LevelRequest>>>,
}

impl InMemoryQueue {
    pub fn new(buffer_size: usize) -> (Self, mpsc::Receiver<LevelRequest>) {
        let (sender, receiver) = mpsc::channel(buffer_size);
        let pending_requests = Arc::new(Mutex::new(Vec::new()));

        let queue = Self {
            sender,
            pending_requests,
        };

        (queue, receiver)
    }

    pub async fn remove_request(&self, request_id: &str) {
        let mut pending = self.pending_requests.lock().await;
        if let Some(pos) = pending.iter().position(|req| req.request_id == request_id) {
            pending.remove(pos);
        }
    }
}

#[async_trait]
impl BadgeUpdateQueue for InMemoryQueue {
    async fn enqueue(&self, mut request: LevelRequest) -> Result<(), String> {
        if request.request_id.is_empty() {
            request.request_id = uuid::Uuid::new_v4().to_string();
        }

        if request.created_at.timestamp() == 0 {
            request.created_at = chrono::Utc::now();
        }

        {
            let mut pending = self.pending_requests.lock().await;
            pending.push(request.clone());
            info!("Queue size: {} requests pending", pending.len());
        }

        // Send to the channel
        self.sender
            .send(request)
            .await
            .map_err(|e| format!("Failed to enqueue badge update request: {}", e))
    }

    async fn get_pending_requests(&self) -> Vec<LevelRequest> {
        let pending = self.pending_requests.lock().await;
        pending.clone()
    }
}
