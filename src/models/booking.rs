use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};
use chrono::NaiveDateTime;
use std::fmt;

use crate::utils::parse_datetime_local;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "bookingstatus", rename_all = "lowercase")]
pub enum BookingStatus {
    Accepted,
    Pending,
    Rejected,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Booking {
    pub booking_id: i32,
    pub class_id: i32,
    pub booking_from: NaiveDateTime,
    pub booking_to: NaiveDateTime,
    pub booking_owner: String,
    pub booking_confirmed: BookingStatus,
}

impl fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookingStatus::Accepted => write!(f, "Accepted"),
            BookingStatus::Pending => write!(f, "Pending"),
            BookingStatus::Rejected => write!(f, "Rejected"),
        }
    }
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