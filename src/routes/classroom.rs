use axum::{extract::State, response::Html};
use sqlx::PgPool;
use askama::Template;

use crate::models::classroom::Classroom;
use crate::db::db_get_all_classrooms;
use crate::routes::templates_structs::AllClassroomsTemplate;

pub async fn get_classrooms(State(pool): State<PgPool>) -> Html<String> {
    let classrooms: Vec<Classroom> = db_get_all_classrooms(&pool).await;
    let template = AllClassroomsTemplate { classrooms };

    Html(template.render().unwrap())
}
