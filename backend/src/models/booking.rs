use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Booking {
    pub id: i32,
    pub class_id: i16,
    // pub from: Date,
    // pub to: Date,
    pub owner: String,
    pub confirmed: bool,
}
