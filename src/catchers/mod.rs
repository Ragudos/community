use rocket::{catch, Error, Request};

#[catch(422)]
pub fn unprocessable_entity(request: &Request) -> &'static str {
    "Please check the information you've entered and try again."
}
