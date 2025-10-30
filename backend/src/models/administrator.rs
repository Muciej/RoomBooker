use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Booking {
    pub id: i16,
    pub name: String,
    pub login: String,
    // pub password: Hash,
}
