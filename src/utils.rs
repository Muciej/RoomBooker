use serde::{Deserializer, Deserialize, Serialize};
use chrono::NaiveDateTime;

pub fn parse_datetime_local<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
D: Deserializer<'de>,
{
    let format: &str = "%Y-%m-%dT%H:%M";
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, format)
        .map_err(serde::de::Error::custom)
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}
