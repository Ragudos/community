use rocket::http::Status;
use rocket_dyn_templates::{context, Metadata};

use crate::models::{api::ApiResponse, Toast, ToastTypes};

pub fn sqlx_error_to_api_response<'r>(
    error: sqlx::Error,
    message: Option<&'r str>,
    metadata: &Metadata<'r>,
) -> ApiResponse {
    eprintln!("SQLX Error: {:?}", error);

    let (mime, html) = metadata
        .render(
            "partials/toast",
            context! {
                toast: Toast {
                    message: message.unwrap_or("Something went wrong. Please try again later.").to_string(),
                    r#type: Some(ToastTypes::Error),
                }
            },
        )
        .unwrap();

    ApiResponse::CustomHTML(Status::InternalServerError, mime, html)
}
