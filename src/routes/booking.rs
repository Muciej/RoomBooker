use std::collections::HashMap;

use askama::Template;
use axum::{
    Json,
    body::Body,
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use sqlx::PgPool;

use crate::db::{
    DBError, db_delete_booking, db_get_all_bookings, db_get_all_classrooms, db_insert_new_booking,
};
use crate::models::booking::{CreateBooking, DeleteBooking, sort_bookings_by_start_date};
use crate::routes::templates_structs::{AddBookingTemplate, AllBookingsTemplate};
use crate::utils::ApiResponse;

pub async fn get_bookings(State(pool): State<PgPool>) -> Html<String> {
    let mut bookings = db_get_all_bookings(&pool).await.unwrap_or(vec![]);
    let classrooms = db_get_all_classrooms(&pool).await.unwrap_or(vec![]);

    sort_bookings_by_start_date(&mut bookings);

    let mut classrooms_id_to_data_map = HashMap::new();
    for c in classrooms {
        let data_str = format!(
            "{}\t{}",
            c.class_name.unwrap_or("".to_string()),
            c.class_number
                .map(|n| n.to_string())
                .unwrap_or("".to_string())
        );
        classrooms_id_to_data_map.insert(c.class_id, data_str);
    }

    let template = AllBookingsTemplate {
        bookings,
        classroom_data: classrooms_id_to_data_map,
    };

    Html(template.render().unwrap())
}

pub async fn add_booking_form(State(pool): State<PgPool>) -> Html<String> {
    let classrooms = db_get_all_classrooms(&pool).await.unwrap_or(vec![]);
    let template = AddBookingTemplate { classrooms };
    Html(template.render().unwrap())
}

pub async fn post_booking(
    State(pool): State<PgPool>,
    Form(input): Form<CreateBooking>,
) -> Response {
    let result = db_insert_new_booking(&pool, input).await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: "Booking created successfully".to_string(),
            }),
        )
            .into_response(),
        Err(e) => match e {
            DBError::InvalidInsert(s) => (
                StatusCode::EXPECTATION_FAILED,
                Json(ApiResponse {
                    success: false,
                    message: s,
                }),
            )
                .into_response(),
            DBError::ConnectionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: "Lost connetion with the database!".to_string(),
                }),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: "Internal server error occured!".to_string(),
                }),
            )
                .into_response(),
        },
    }
}

pub async fn delete_booking(
    State(pool): State<PgPool>,
    Form(delete_booking): Form<DeleteBooking>,
) -> Response {
    let result = db_delete_booking(&pool, delete_booking).await;
    if let Ok(_) = result {
        (StatusCode::OK, Body::empty()).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, Body::empty()).into_response()
    }
}
