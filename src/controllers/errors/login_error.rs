use rocket::{
    form::{error::ErrorKind, Errors, Form},
    http::Status,
};
use rocket_dyn_templates::{context, Metadata};

use crate::models::{api::ApiResponse, users::form::LoginFormData, Toast};

pub fn render_error<'r>(
    metadata: &Metadata<'r>,
    status: Status,
    name_error: Option<&'r str>,
    password_error: Option<&'r str>,
    honeypot_error: Option<Toast>,
) -> ApiResponse {
    let (mime, html) = metadata
        .render(
            "partials/auth/login_error",
            context! {
                name_error,
                password_error,
                toast: honeypot_error,
            },
        )
        .unwrap();

    ApiResponse::CustomHTML(status, mime, html)
}

pub fn get_login_data_or_return_validation_error<'r>(
    metadata: &Metadata<'r>,
    login_data: Result<Form<LoginFormData<'r>>, Errors<'r>>,
) -> Result<LoginFormData<'r>, ApiResponse> {
    Ok(login_data
        .map_err(|errors| {
            let mut name_error: Option<String> = None;
            let mut password_error: Option<String> = None;
            let mut honeypot_error: Option<Toast> = None;

            for error in errors.into_iter() {
                let is_for_name = error.is_for_exactly("username");
                let is_for_password = error.is_for_exactly("password");
                let is_for_honeypot = error.is_for_exactly("honeypot");

                if is_for_name {
                    match error.kind {
                        ErrorKind::Missing => {
                            name_error = Some("Username is required".to_string());
                        },
                        _ => {
                            name_error = Some(error.kind.to_string());
                        }
                    }
                }

                if is_for_password {
                    match error.kind {
                        ErrorKind::Missing => {
                            password_error = Some("Password is required".to_string());
                        },
                        _ => {
                            password_error = Some(error.kind.to_string());
                        }
                    }
                }

                if is_for_honeypot {
                    honeypot_error = Some(Toast {
                        message: error.kind.to_string(),
                        r#type: Some(crate::models::ToastTypes::Error),
                    });
                }
            }

            render_error(
                metadata,
                Status::UnprocessableEntity,
                name_error.as_deref(),
                password_error.as_deref(),
                honeypot_error,
            )
        })?
        .into_inner())
}
