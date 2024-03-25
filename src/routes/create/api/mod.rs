use rocket::http::Status;
use rocket::post;

pub mod community;

#[post("/<_..>", rank = 2)]
pub fn logged_out<'r>() -> Status {
    Status::Unauthorized
}
