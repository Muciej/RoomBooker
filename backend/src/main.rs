use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

mod db;
mod routes;
mod models;

use routes::classroom::get_classrooms;

#[tokio::main]
async fn main() {
    // Initialize database
    let pool = db::connect().await;

    // Build routes
    let app = Router::new()
        .route("/api/classrooms", get(get_classrooms))
        .with_state(pool)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
