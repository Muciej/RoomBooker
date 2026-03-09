use sqlx::{
    PgPool, Pool, Postgres,
    postgres::{PgPoolOptions, PgQueryResult},
};
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::models::{
    booking::{Booking, CreateBooking, DeleteBooking},
    classroom::Classroom,
};

#[derive(Debug)]
pub enum DBError {
    ConnectionError,
    InvalidInsert(String),
    InvalidDeleteCodeError,
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

pub async fn db_get_filtered_bookings(
    pool: &Pool<Postgres>,
    predicate: impl Fn(&Booking) -> bool,
) -> Result<Vec<Booking>, DBError> {
    let all_bookings = db_get_all_bookings(pool).await?;
    Ok(all_bookings
        .into_iter()
        .filter(predicate)
        .collect::<Vec<Booking>>())
}

fn get_hash(input: &String) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish().to_string()
}

pub async fn db_insert_new_booking(
    pool: &Pool<Postgres>,
    new_booking: CreateBooking,
) -> Result<PgQueryResult, DBError> {
    let colliding_bookings = db_get_filtered_bookings(pool, |booking| {
        booking.class_id == new_booking.class_id
            && booking.booking_to > new_booking.booking_from
            && booking.booking_from < new_booking.booking_to
    })
    .await?;

    if colliding_bookings.len() > 0 {
        return Err(DBError::InvalidInsert(
            "Booking overlaps with another one!".to_string(),
        ));
    }

    let booking_duration = new_booking.booking_to - new_booking.booking_from;
    if booking_duration.num_minutes() > 8 * 60 {
        return Err(DBError::InvalidInsert(
            "Maximal allowed booking duration is 8 hours!".to_string(),
        ));
    }

    if new_booking.booking_to <= new_booking.booking_from {
        return Err(DBError::InvalidInsert(
            "Invalid end of the booking!".to_string(),
        ));
    }

    sqlx::query!(
        r#"
        INSERT INTO bookings
            (booking_id, class_id, booking_from, booking_to, booking_owner, booking_confirmed, booking_delete_hash)
        VALUES (DEFAULT, $1, $2, $3, $4, 'accepted', $5)
        "#,
        new_booking.class_id,
        new_booking.booking_from,
        new_booking.booking_to,
        new_booking.booking_owner,
        get_hash(&new_booking.booking_delete_code),
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

pub async fn db_delete_booking(
    pool: &Pool<Postgres>,
    booking_to_delete: DeleteBooking,
) -> Result<PgQueryResult, DBError> {
    let admin_hash = env::var("ADMIN_PWD_HASH").unwrap_or_default();
    let booking =
        db_get_filtered_bookings(pool, |booking| booking.booking_id == booking_to_delete.booking_id)
            .await?;
    let entered_pwd_hash = get_hash(&booking_to_delete.delete_code);

    if booking[0].booking_delete_hash == entered_pwd_hash || entered_pwd_hash == admin_hash {
        sqlx::query!(
            r#"
                    DELETE FROM bookings WHERE booking_id = $1
                    "#,
            booking_to_delete.booking_id,
        )
        .execute(pool)
        .await
        .map_err(|_| DBError::DBInternalError)
    } else {
        Err(DBError::InvalidDeleteCodeError)
    }
}
