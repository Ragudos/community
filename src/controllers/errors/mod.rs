use rocket::form::{Errors, Form, FromForm};
use rocket::http::Status;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

use crate::models::{Toast, ToastTypes};
use crate::responders::ApiResponse;

#[derive(Deserialize, Serialize)]
pub struct ValidationError {
    pub field: Option<String>,
    pub message: String,
}

pub fn extract_data_or_return_response<'r, T>(
    data: Result<Form<T>, Errors<'r>>,
    error_template_name: &'static str,
) -> Result<T, ApiResponse>
where
    T: FromForm<'r>,
{
    Ok(data
        .map_err(|errors| {
            let errors = errors
                .into_iter()
                .map(|error| ValidationError {
                    field: error.name.map(|name| name.to_string()),
                    message: error.kind.to_string(),
                })
                .collect::<Vec<ValidationError>>();

            ApiResponse::Render {
                status: Status::UnprocessableEntity,
                template: Some(Template::render(error_template_name, context! { errors })),
                headers: None,
            }
        })?
        .into_inner())
}

impl<T> From<T> for ApiResponse
where
    T: std::error::Error,
{
    fn from(error: T) -> Self {
        eprintln!("{:?}", error);

        ApiResponse::Render {
            status: Status::InternalServerError,
            template: Some(Template::render(
                "partials/toast",
                context! {
                    toast: Toast {
                        message: "Something went wrong. Please try again later.".to_string(),
                        r#type: Some(ToastTypes::Error)
                    }
                },
            )),
            headers: None,
        }
    }
}
