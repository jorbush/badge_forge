use tracing::Subscriber;
use tracing::field::Visit;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;
use tracing_subscriber::{prelude::*, registry::Registry};

struct JsonVisitor {
    fields: serde_json::Map<String, serde_json::Value>,
}

impl Visit for JsonVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.fields.insert(
            field.name().to_string(),
            serde_json::Value::String(format!("{:?}", value)),
        );
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.fields.insert(
            field.name().to_string(),
            serde_json::Value::String(value.to_string()),
        );
    }
}

struct AxiomLoggingLayer {
    sender: tokio::sync::mpsc::UnboundedSender<serde_json::Value>,
    service: &'static str,
}

impl<S> Layer<S> for AxiomLoggingLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = JsonVisitor {
            fields: serde_json::Map::new(),
        };
        event.record(&mut visitor);

        let metadata = event.metadata();
        let level = metadata.level().to_string().to_lowercase();

        let message = visitor
            .fields
            .remove("message")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();

        let mut payload = serde_json::json!({
            "message": message,
            "level": level,
            "service": self.service,
            "_time": chrono::Utc::now().to_rfc3339(),
        });

        if let Some(obj) = payload.as_object_mut() {
            for (k, v) in visitor.fields {
                obj.insert(k, v);
            }
        }

        let _ = self.sender.send(payload);
    }
}

async fn flush_logs(
    buffer: &[serde_json::Value],
    client: &reqwest::Client,
    dataset: &str,
    token: &str,
) {
    let url = format!("https://api.axiom.co/v1/datasets/{}/ingest", dataset);
    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(buffer)
        .send()
        .await;

    if let Err(e) = res {
        eprintln!("[axiom-logging] Failed to send logs to Axiom: {}", e);
    } else if let Ok(resp) = res {
        if !resp.status().is_success() {
            if let Ok(text) = resp.text().await {
                eprintln!("[axiom-logging] Failed to send logs to Axiom: {}", text);
            }
        }
    }
}

pub fn init_logging(service_name: &'static str) {
    let fmt_layer =
        tracing_subscriber::fmt::layer().with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    let axiom_token = std::env::var("AXIOM_TOKEN").ok();
    let axiom_dataset = std::env::var("AXIOM_DATASET").ok();

    let axiom_layer = if let (Some(token), Some(dataset)) = (axiom_token, axiom_dataset) {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<serde_json::Value>();

        // Spawn background worker to batch-send logs
        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let mut buffer = Vec::new();
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

            loop {
                tokio::select! {
                    Some(event) = rx.recv() => {
                        buffer.push(event);
                        if buffer.len() >= 100 {
                            flush_logs(&buffer, &client, &dataset, &token).await;
                            buffer.clear();
                        }
                    }
                    _ = interval.tick() => {
                        if !buffer.is_empty() {
                            flush_logs(&buffer, &client, &dataset, &token).await;
                            buffer.clear();
                        }
                    }
                }
            }
        });

        Some(
            AxiomLoggingLayer {
                sender: tx,
                service: service_name,
            }
            .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
    } else {
        None
    };

    Registry::default().with(axiom_layer).with(fmt_layer).init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logging_layer_formats_properly() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<serde_json::Value>();

        let layer = AxiomLoggingLayer {
            sender: tx,
            service: "test_service",
        };

        let subscriber = Registry::default().with(layer);

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!(some_key = "some_val", "Hello from test");
        });

        // Pull the event from the receiver channel
        let event = rx.try_recv().expect("Should have received an event");

        assert_eq!(event["message"], "Hello from test");
        assert_eq!(event["level"], "info");
        assert_eq!(event["service"], "test_service");
        assert_eq!(event["some_key"], "some_val");
        assert!(event["_time"].is_string());
    }
}
