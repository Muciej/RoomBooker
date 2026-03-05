use sqlx::{PgPool, Pool, Postgres, postgres::{PgPoolOptions, PgQueryResult}};

use crate::models::{booking::{Booking, CreateBooking}, classroom::Classroom};

#[derive(Debug)]
pub enum DBError {
    ConnectionError,
    InvalidInsert(String),
    DBInternalError,
}

pub async fn connect(database_url: &String) -> Result<PgPool, DBError> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|_| DBError::ConnectionError)
}

pub async fn db_get_all_bookings(pool: &Pool<Postgres>) -> Result<Vec<Booking>, DBError> {
    sqlx::query_as::<_, Booking>("SELECT * FROM bookings")
        .fetch_all(pool)
        .await
        .map_err(|_| DBError::DBInternalError)
}

pub async fn db_get_filtered_bookings(pool: &Pool<Postgres>, predicate: impl Fn(&Booking) -> bool) -> Result<Vec<Booking>, DBError> {
    let all_bookings = db_get_all_bookings(pool).await?;
    Ok(all_bookings.into_iter().filter(predicate).collect::<Vec<Booking>>())
}

pub async fn db_insert_new_booking(pool: &Pool<Postgres>, new_booking: CreateBooking) -> Result<PgQueryResult, DBError> {

    let colliding_bookings = db_get_filtered_bookings(pool, |booking| {
        booking.class_id == new_booking.class_id &&
        booking.booking_to > new_booking.booking_from &&
        booking.booking_from < new_booking.booking_to
    } ).await?;

    if colliding_bookings.len() > 0 {
        return Err(DBError::InvalidInsert("Booking overlaps with another one!".to_string()));
    }

    let booking_duration = new_booking.booking_to - new_booking.booking_from;
    if booking_duration.num_minutes() > 8 * 60 {
        return Err(DBError::InvalidInsert("Maximal allowed booking duration is 8 hours!".to_string()))
    }

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
    .map_err(|_| DBError::DBInternalError)
}

pub async fn db_get_all_classrooms(pool: &Pool<Postgres>) -> Result<Vec<Classroom>, DBError> {
    sqlx::query_as::<_, Classroom>("SELECT * FROM classrooms")
        .fetch_all(pool)
        .await
        .map_err(|_| DBError::DBInternalError)
}
