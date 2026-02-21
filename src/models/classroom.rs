use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Classroom {
    pub class_id: i32,
    pub class_name: Option<String>,
    pub class_number: Option<i32>,
}
