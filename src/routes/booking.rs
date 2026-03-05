use askama::Template;
use axum::{
    extract::{Form, State},
    response::Html,
};
use sqlx::PgPool;

use crate::db::{DBError, db_get_all_bookings, db_insert_new_booking};
use crate::models::booking::CreateBooking;
use crate::routes::templates_structs::{AddBookingTemplate, AllBookingsTemplate};

pub async fn get_bookings(State(pool): State<PgPool>) -> Html<String> {
    let mut bookings = db_get_all_bookings(&pool).await.unwrap_or(vec![]);
    bookings.sort_by(|b1, b2| {
        if b1.booking_from == b2.booking_from {
            return std::cmp::Ordering::Equal;
        } else {
            match b1.booking_from < b2.booking_from {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater,
            }
        }
    });

    let template = AllBookingsTemplate { bookings };

    Html(template.render().unwrap())
}

pub async fn add_booking_form() -> Html<String> {
    let template = AddBookingTemplate { error_msg: None };
    Html(template.render().unwrap())
}

pub async fn post_booking(
    State(pool): State<PgPool>,
    Form(input): Form<CreateBooking>,
) -> Html<String> {
    let result = db_insert_new_booking(&pool, input).await;

    match result {
        Ok(_) => Html(
            AddBookingTemplate {
                error_msg: Some("Booking created!".into()),
            }
            .render()
            .unwrap(),
        ),
        Err(e) => match e {
            DBError::InvalidInsert(s) => Html(
                AddBookingTemplate {
                    error_msg: Some(format!("Error: {}", s)),
                }
                .render()
                .unwrap(),
            ),
            DBError::DBInternalError => Html(
                AddBookingTemplate {
                    error_msg: Some("Internal database error occured!".to_owned()),
                }
                .render()
                .unwrap(),
            ),
            DBError::ConnectionError => Html(
                AddBookingTemplate {
                    error_msg: Some("Lost connection with the database!".to_owned()),
                }
                .render()
                .unwrap(),
            ),
        },
    }
}
