use rocket::http::Status;
use rocket_dyn_templates::{context, Metadata};

use crate::models::{api::ApiResponse, Toast, ToastTypes};

pub fn bcrypt_error_to_api_response<'r>(
    metadata: &Metadata<'r>,
    error: bcrypt::BcryptError,
    message: Option<&'r str>,
) -> ApiResponse {
    eprintln!("Error in bcrypt: {:?}", error);

    let (mime, html) = metadata
        .render(
            "partials/toast",
            context! {
                toast: Toast {
                    message: message.unwrap_or("Something went wrong. Please try again later.").to_string(),
                    r#type: Some(ToastTypes::Error)
                }
            },
        )
        .unwrap();

    ApiResponse::CustomHTML(Status::InternalServerError, mime, html)
}
