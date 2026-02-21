use dotenvy::dotenv;
use sqlx::{PgPool, Pool, Postgres, postgres::PgPoolOptions};
use std::env;

use crate::models::{booking::Booking, classroom::Classroom};

pub async fn connect() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

pub async fn get_all_bookings(pool: &Pool<Postgres>) -> Vec<Booking> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|_| vec![])
}

pub async fn get_all_classrooms(pool: &Pool<Postgres>) -> Vec<Classroom> {
    sqlx::query_as::<_, Classroom>("SELECT * FROM classrooms")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|_| vec![])
}
