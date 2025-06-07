use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
