use askama::Template;

#[derive(Template)]
#[template(path = "add_booking.html")]
pub struct AddBookingTemplate {
    pub error_msg: Option<String>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}
