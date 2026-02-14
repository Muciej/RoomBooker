use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};
use chrono::NaiveDateTime;

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
