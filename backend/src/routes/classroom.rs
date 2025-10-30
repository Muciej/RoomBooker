use axum::{extract::State, response::Json};
use sqlx::PgPool;
use serde_json::json;

use crate::models::classroom::Classroom;

pub async fn get_classrooms(State(pool): State<PgPool>) -> Json<serde_json::Value> {
    let rows = sqlx::query_as::<_, Classroom>("SELECT id, name, capacity FROM classrooms")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Json(json!({ "classrooms": rows }))
}
