use axum::response::Html;
use askama::Template;
use crate::routes::templates_structs::PrivacyPolicyTemplate;

pub async fn get_privacy_policy() -> Html<String> {
    let template = PrivacyPolicyTemplate{};
    Html(template.render().unwrap())
}
