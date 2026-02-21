use axum::{
    extract::{State, Form},
    response::Html,
};
use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer};
use sqlx::PgPool;
use askama::Template;

use crate::db::get_all_bookings;
use crate::routes::templates_structs::{AddBookingTemplate, AllBookingsTemplate};

pub async fn get_bookings(State(pool): State<PgPool>) -> Html<String> {
    let bookings = get_all_bookings(&pool).await;

    let template = AllBookingsTemplate { bookings };

    Html(template.render().unwrap())
}

pub async fn add_booking_form() -> Html<String> {
    let template = AddBookingTemplate{error_msg: None};
    Html(template.render().unwrap())
}

fn parse_datetime_local<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
D: Deserializer<'de>,
{
    let format: &str = "%Y-%m-%dT%H:%M";
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, format)
        .map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct CreateBooking {
    pub class_id: i32,

    #[serde(deserialize_with = "parse_datetime_local")]
    pub booking_from: NaiveDateTime,

    #[serde(deserialize_with = "parse_datetime_local")]
    pub booking_to: NaiveDateTime,
    
    pub booking_owner: String,
}

pub async fn post_booking(
    State(pool): State<PgPool>,
    Form(input): Form<CreateBooking>,
) -> Html<String> {
    let result = sqlx::query!(
        r#"
        INSERT INTO bookings
            (booking_id, class_id, booking_from, booking_to, booking_owner, booking_confirmed)
        VALUES (DEFAULT, $1, $2, $3, $4, 'accepted')
        "#,
        input.class_id,
        input.booking_from,
        input.booking_to,
        input.booking_owner
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Html(AddBookingTemplate{ error_msg: Some("Booking created!".into())}.render().unwrap()),
        Err(e) => Html(AddBookingTemplate{ error_msg: Some(format!("Error: {}", e))}.render().unwrap()),
    }
}
