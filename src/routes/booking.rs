use askama::Template;
use axum::{
    extract::{Form, State},
    response::Html,
};
use sqlx::PgPool;

use crate::db::{DBError, db_get_all_bookings, db_get_all_classrooms, db_insert_new_booking};
use crate::models::booking::{CreateBooking, sort_bookings_by_start_date};
use crate::routes::templates_structs::{AddBookingTemplate, AllBookingsTemplate};

pub async fn get_bookings(State(pool): State<PgPool>) -> Html<String> {
    let mut bookings = db_get_all_bookings(&pool).await.unwrap_or(vec![]);
    sort_bookings_by_start_date(&mut bookings);

    let template = AllBookingsTemplate { bookings };

    Html(template.render().unwrap())
}

pub async fn add_booking_form(State(pool): State<PgPool>) -> Html<String> {
    let classrooms = db_get_all_classrooms(&pool).await.unwrap_or(vec![]);
    let template = AddBookingTemplate { error_msg: None, classrooms };
    Html(template.render().unwrap())
}

pub async fn post_booking(
    State(pool): State<PgPool>,
    Form(input): Form<CreateBooking>,
) -> Html<String> {
    let classrooms = db_get_all_classrooms(&pool).await.unwrap_or(vec![]);
    let result = db_insert_new_booking(&pool, input).await;

    match result {
        Ok(_) => Html(
            AddBookingTemplate {
                error_msg: Some("Booking created!".into()),
                classrooms
            }
            .render()
            .unwrap(),
        ),
        Err(e) => match e {
            DBError::InvalidInsert(s) => Html(
                AddBookingTemplate {
                    error_msg: Some(format!("Error: {}", s)),
                    classrooms
                }
                .render()
                .unwrap(),
            ),
            DBError::DBInternalError => Html(
                AddBookingTemplate {
                    error_msg: Some("Internal database error occured!".to_owned()),
                    classrooms
                }
                .render()
                .unwrap(),
            ),
            DBError::ConnectionError => Html(
                AddBookingTemplate {
                    error_msg: Some("Lost connection with the database!".to_owned()),
                    classrooms
                }
                .render()
                .unwrap(),
            ),
        },
    }
}
