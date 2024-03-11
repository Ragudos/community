use rocket::http::Status;
use rocket_dyn_templates::{context, Metadata};
use serde_json::Error;

use crate::models::{api::ApiResponse, Toast, ToastTypes};

pub fn serde_json_error_to_api_response<'r>(
    metadata: &Metadata<'r>,
    error: Error,
    status: Status,
    message: &'r str,
) -> ApiResponse {
    eprintln!("Serde JSON Error: {:?}", error);

    let (mime, html) = metadata
        .render(
            "partials/components/toast",
            context! {
                toast: Toast {
                    message: message.to_string(),
                    r#type: Some(ToastTypes::Error)
                }
            },
        )
        .unwrap();

    ApiResponse::CustomHTML(status, mime, html)
}
