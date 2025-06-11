mod api;
mod model;
mod utils;

use api::route::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();
    println!("Badge Forge API started successfully on port 4000 🎖️");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
