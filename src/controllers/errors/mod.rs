use std::num::ParseIntError;

use rocket::form::{Errors, Form, FromForm};
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
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
                template: Some(Template::render(
                    error_template_name,
                    context! { errors },
                )),
                headers: None,
            }
        })?
        .into_inner())
}

impl From<Box<dyn std::error::Error>> for ApiResponse {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        eprintln!("{:?}", error);
        ApiResponse::Status(Status::InternalServerError)
    }
}

impl From<sqlx::Error> for ApiResponse {
    fn from(error: sqlx::Error) -> Self {
        eprintln!("{:?}", error);
        ApiResponse::Status(Status::InternalServerError)
    }
}

impl From<serde_json::Error> for ApiResponse {
    fn from(error: serde_json::Error) -> Self {
        eprintln!("{:?}", error);
        ApiResponse::Status(Status::UnprocessableEntity)
    }
}

impl From<bcrypt::BcryptError> for ApiResponse {
    fn from(error: bcrypt::BcryptError) -> Self {
        eprintln!("{:?}", error);
        ApiResponse::Status(Status::InternalServerError)
    }
}

impl From<ParseIntError> for ApiResponse {
    fn from(error: ParseIntError) -> Self {
        eprintln!("{:?}", error);
        ApiResponse::Status(Status::UnsupportedMediaType)
    }
}
