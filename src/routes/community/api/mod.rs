use rocket::{get, http::Status};

pub mod join;
pub mod rename;
pub mod request_deletion;
pub mod settings;
pub mod delete_community;
pub mod request_change_join_process;
pub mod change_join_process;

#[get("/<_..>", rank = 3)]
pub fn logged_out() -> Status {
    Status::Unauthorized
}
