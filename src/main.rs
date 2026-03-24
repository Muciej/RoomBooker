use std::net::SocketAddr;
use sqlx::{Pool, Postgres};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use axum::{
    Router,
    routing::{post, get},
};
use routes::{
    booking::{add_booking_form, get_bookings, post_booking, delete_booking},
    classroom::get_classrooms,
    main_page::{get_index, get_root}
};
use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;
use std::env;

mod utils;
mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // Load env file
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    // Initialize database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect(&database_url).await.expect("Couldn't connect to the database!");

    // Build routes
    let app = build_app(pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

fn build_app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(get_root))
        .route("/index.html", get(get_index))
        .route("/classrooms", get(get_classrooms))
        .route("/bookings", get(get_bookings).post(post_booking))
        .route("/bookings/new", get(add_booking_form))
        .route("/bookings/delete", post(delete_booking))
        .with_state(pool)
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}