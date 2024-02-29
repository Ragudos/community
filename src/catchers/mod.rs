use rocket::{catch, Request};

#[catch(422)]
pub fn unprocessable_entity(_request: &Request) -> &'static str {
    "Please check the information you've entered and try again."
}
