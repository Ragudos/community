use rocket::{form::Form, http::Status};
use rocket_dyn_templates::{context, Metadata};

use crate::models::{api::ApiResponse, users::form::RegisterFormData, Toast, ToastTypes};

pub fn render_error<'r>(
    metadata: &Metadata<'r>,
    status: Status,
    name_error: Option<&'r str>,
    password_error: Option<&'r str>,
    gender_error: Option<&'r str>,
    honeypot_error: Option<Toast>,
) -> ApiResponse {
    let (mime, html) = metadata
        .render(
            "partials/components/register/error",
            context! {
                name_error,
                password_error,
                gender_error,
                toast: honeypot_error,
            },
        )
        .unwrap();

    ApiResponse::CustomHTML(status, mime, html)
}

pub fn get_register_data_or_return_validation_errors<'r>(
    metadata: &Metadata<'r>,
    register_data: Result<Form<RegisterFormData<'r>>, rocket::form::Errors<'r>>,
) -> Result<RegisterFormData<'r>, ApiResponse> {
    Ok(register_data
        .map_err(|errors| {
            let mut name_error: Option<String> = None;
            let mut password_error: Option<String> = None;
            let mut gender_error: Option<String> = None;
            let mut honeypot_error: Option<Toast> = None;

            for error in errors.into_iter() {
                let is_for_name = error.is_for_exactly("display_name");
                let is_for_password = error.is_for_exactly("password");
                let is_for_gender = error.is_for_exactly("gender");
                let is_for_honeypot = error.is_for_exactly("honeypot");

                if is_for_name {
                    name_error = Some(error.kind.to_string());
                }

                if is_for_password {
                    password_error = Some(error.kind.to_string());
                }

                if is_for_gender {
                    gender_error = Some(error.kind.to_string());
                }

                if is_for_honeypot {
                    honeypot_error = Some(Toast {
                        message: error.kind.to_string(),
                        r#type: Some(ToastTypes::Error),
                    });
                }
            }

            render_error(
                metadata,
                Status::BadRequest,
                name_error.as_deref(),
                password_error.as_deref(),
                gender_error.as_deref(),
                honeypot_error,
            )
        })?
        .into_inner())
}
