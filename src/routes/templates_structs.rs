use std::collections::HashMap;

use askama::Template;

use crate::models::{booking::Booking, classroom::Classroom};

#[derive(Template)]
#[template(path = "add_booking.html")]
pub struct AddBookingTemplate {
    pub classrooms: Vec<Classroom>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "privacy_policy.html")]
pub struct PrivacyPolicyTemplate {}

#[derive(Template)]
#[template(path = "all_bookings.html")]
pub struct AllBookingsTemplate {
    pub bookings: Vec<Booking>,
    pub classroom_data: HashMap<i32, String>,
}

#[derive(Template)]
#[template(path = "all_classrooms.html")]
pub struct AllClassroomsTemplate {
    pub classrooms: Vec<Classroom>,
}
