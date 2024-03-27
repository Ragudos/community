use rocket::{get, http::Status};
use rocket_dyn_templates::{context, Template};

use crate::{
    models::{users::schema::UserJWT, Toast, ToastTypes},
    responders::ApiResponse,
};

pub mod join;
pub mod rename;
pub mod request_deletion;
pub mod settings;

#[get("/<_..>", rank = 3)]
pub fn logged_out() -> Status {
    Status::NoContent
}
