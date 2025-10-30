use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Classroom {
    pub id: i16,
    pub name: String,
    pub class_number: i32,
}
