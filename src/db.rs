use dotenvy::dotenv;
use sqlx::{PgPool, Pool, Postgres, postgres::{PgPoolOptions, PgQueryResult}};
use std::env;

use crate::models::{booking::{Booking, CreateBooking}, classroom::Classroom};

pub async fn connect() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

pub async fn db_get_all_bookings(pool: &Pool<Postgres>) -> Vec<Booking> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|_| vec![])
}

pub async fn db_insert_new_booking(pool: &Pool<Postgres>, new_booking: CreateBooking) -> sqlx::Result<PgQueryResult> {
    sqlx::query!(
        r#"
        INSERT INTO bookings
            (booking_id, class_id, booking_from, booking_to, booking_owner, booking_confirmed)
        VALUES (DEFAULT, $1, $2, $3, $4, 'accepted')
        "#,
        new_booking.class_id,
        new_booking.booking_from,
        new_booking.booking_to,
        new_booking.booking_owner
    )
    .execute(pool)
    .await
}

pub async fn db_get_all_classrooms(pool: &Pool<Postgres>) -> Vec<Classroom> {
    sqlx::query_as::<_, Classroom>("SELECT * FROM classrooms")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|_| vec![])
}
