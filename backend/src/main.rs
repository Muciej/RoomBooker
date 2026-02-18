use std::net::SocketAddr;
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
use axum::{
    Router,
    routing::{get, post},
};
use routes::{
    booking::{add_booking_form, get_bookings, post_booking},
    classroom::get_classrooms
};

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt().with_env_filter("debug").init();

    // Initialize database
    let pool = db::connect().await;

    // Build routes
    let app = build_app(pool);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

fn build_app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/classrooms", get(get_classrooms))
        .route("/bookings", get(get_bookings).post(post_booking))
        .route("/bookings/new", get(add_booking_form))
        .with_state(pool)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}