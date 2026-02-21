use axum::{extract::State, response::Html};
use sqlx::PgPool;

use crate::models::classroom::Classroom;

pub async fn get_classrooms(State(pool): State<PgPool>) -> Html<String> {
    let classrooms: Vec<Classroom> = sqlx::query_as::<_, Classroom>("SELECT * FROM classrooms")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);

    let mut html = String::from("<h1>Classrooms</h1><ul>");


    for c in classrooms {

        html.push_str(&format!(
            "<li>ID: {} | Name: {:?} | Number: {:?}</li>",
            c.class_id,
            c.class_name.unwrap_or_else(|| "-".to_string()),
            c.class_number
        ));
    }

    html.push_str("</ul>");

    Html(html)
}
