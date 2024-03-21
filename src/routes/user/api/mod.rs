use rocket::{get, http::Status};

pub mod img_name;

#[get("/<_..>", rank = 2)]
pub fn malformed_uri_or_logged_out() -> Status {
    Status::NoContent
}
