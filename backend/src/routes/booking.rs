use axum::{
    extract::{State, Form},
    http::StatusCode,
    response::Html,
};
use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer};
use sqlx::PgPool;
use tracing::info;
use askama::Template;

use crate::models::booking::Booking;

pub async fn get_bookings(State(pool): State<PgPool>) -> Html<String> {
    let bookings: Vec<Booking> = sqlx::query_as::<_, Booking>("SELECT * FROM bookings")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    let mut html = String::from("<h1>Bookings</h1><ul>");
    html.push_str(&format!(
        "<li> Total number of bookings: {}",
        bookings.len()
    ));

    for b in bookings {
        html.push_str(&format!(
            "<li>ID: {} | Owner: {} | Booked class ID: {} | From: {:?} | To: {:?} | Booking status {:?}</li>",
            b.booking_id,
            b.booking_owner,
            b.class_id,
            b.booking_from,
            b.booking_to,
            b.booking_confirmed
        ));
    }

    html.push_str("</ul>");

    Html(html)
}

#[derive(Template)]
#[template(path = "add_booking.html")]
struct AddBookingTemplate {
    error_msg: Option<String>,
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
