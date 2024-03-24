use rocket::{http::Status, post};

use crate::responders::ApiResponse;

pub mod private;
pub mod public;

#[post("/<_..>", rank = 2)]
pub fn logged_out() -> Status {
    Status::NoContent
}
