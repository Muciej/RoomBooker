use axum::response::{Html, Redirect};
use askama::Template;
use crate::routes::templates_structs::IndexTemplate;

pub async fn get_root() -> Redirect {
    Redirect::temporary("/index.html")
}

pub async fn get_index() -> Html<String> {
    let template = IndexTemplate{};
    Html(template.render().unwrap())
}
