use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Administrator {
    pub admin_id: i32,
    pub admin_name: Option<String>,
    pub admin_login: String,
    pub admin_password: String,
}
