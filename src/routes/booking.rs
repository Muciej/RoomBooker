use axum::{
    extract::{State, Form},
    response::Html,
};
use sqlx::PgPool;
use askama::Template;

use crate::db::{db_get_all_bookings, db_insert_new_booking};
use crate::models::booking::CreateBooking;
use crate::routes::templates_structs::{AddBookingTemplate, AllBookingsTemplate};

pub async fn get_bookings(State(pool): State<PgPool>) -> Html<String> {
    let bookings = db_get_all_bookings(&pool).await;

    let template = AllBookingsTemplate { bookings };

    Html(template.render().unwrap())
}

pub async fn add_booking_form() -> Html<String> {
    let template = AddBookingTemplate{error_msg: None};
    Html(template.render().unwrap())
}

pub async fn post_booking(
    State(pool): State<PgPool>,
    Form(input): Form<CreateBooking>,
) -> Html<String> {

    let result = db_insert_new_booking(&pool, input).await;

    match result {
        Ok(_) => Html(AddBookingTemplate{ error_msg: Some("Booking created!".into())}.render().unwrap()),
        Err(e) => Html(AddBookingTemplate{ error_msg: Some(format!("Error: {}", e))}.render().unwrap()),
    }
}
