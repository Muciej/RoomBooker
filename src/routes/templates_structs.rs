use askama::Template;

use crate::models::{booking::Booking, classroom::Classroom};

#[derive(Template)]
#[template(path = "add_booking.html")]
pub struct AddBookingTemplate {
    pub error_msg: Option<String>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}


#[derive(Template)]
#[template(path = "all_bookings.html")]
pub struct AllBookingsTemplate {
    pub bookings: Vec<Booking>,
}

#[derive(Template)]
#[template(path = "all_classrooms.html")]
pub struct AllClassroomsTemplate {
    pub classrooms: Vec<Classroom>,
}
