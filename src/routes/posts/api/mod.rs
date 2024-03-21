use rocket::get;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

use crate::{
    models::{users::schema::UserJWT, Toast, ToastTypes},
    responders::ApiResponse,
};

pub mod community_posts;
pub mod post_info;

#[get("/<_..>", rank = 2)]
pub fn malformed_uid(_user: UserJWT) -> ApiResponse {
    ApiResponse::Render {
        status: Status::BadRequest,
        template: Some(Template::render(
            "partials/toast",
            context! {
                toast: Toast {
                    message: "Invalid community UID.".to_string(),
                    r#type: Some(ToastTypes::Error),
                }
            },
        )),
        headers: None,
    }
}

/// Just a no content for any request made where the first
/// endpoint has forwarded.
#[get("/<_..>", rank = 3)]
pub fn logged_out() -> Status {
    Status::NoContent
}
