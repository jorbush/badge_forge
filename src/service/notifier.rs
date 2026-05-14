use reqwest::Client;
use tracing::{error, info};

#[derive(Clone)]
pub struct Notifier {
    client: Client,
    url: String,
    api_key: String,
}

impl Notifier {
    pub fn new(url: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| {
                error!("Failed to build reqwest client with timeout, using default client");
                Client::new()
            });

        Self {
            client,
            url: url.trim_end_matches('/').to_string(),
            api_key,
        }
    }

    pub fn from_env() -> Self {
        let notifier_url = std::env::var("NOTIFIER_URL").unwrap_or_default();
        let notifier_api_key = std::env::var("NOTIFIER_API_KEY").unwrap_or_default();
        Self::new(notifier_url, notifier_api_key)
    }

    pub async fn send_notification(
        &self,
        notification_type: &str,
        recipient: &str,
        metadata: serde_json::Value,
    ) {
        if self.url.is_empty() {
            info!(
                "NOTIFIER_URL is not set or empty, skipping {} notification",
                notification_type
            );
            return;
        }

        let payload = serde_json::json!({
            "type": notification_type,
            "recipient": recipient,
            "metadata": metadata
        });

        let url = format!("{}/notifications", self.url);

        let result = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .json(&payload)
            .send()
            .await;

        match result {
            Ok(resp) => {
                if resp.status().is_success() {
                    info!(
                        "Successfully sent {} notification to {}",
                        notification_type, recipient
                    );
                } else {
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    error!(
                        "Failed to send {} notification. Status: {}, Body: {}",
                        notification_type, status, body
                    );
                }
            }
            Err(e) => error!("Failed to send {} notification: {}", notification_type, e),
        }
    }
}
