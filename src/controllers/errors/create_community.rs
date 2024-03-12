use rocket::{
    form::{Errors, Form},
    http::Status,
};
use rocket_dyn_templates::{context, Metadata};

use crate::models::{api::ApiResponse, community::forms::CreateCommunity, Toast, ToastTypes};

pub fn render_error<'r>(
    metadata: &Metadata<'r>,
    status: Status,
    community_name_error: Option<String>,
    description_error: Option<String>,
    honeypot_error: Option<Toast>,
) -> ApiResponse {
    let (mime, html) = metadata
        .render(
            "partials/components/community/create-community-errors",
            context! {
                community_name_error,
                description_error,
                toast: honeypot_error
            },
        )
        .unwrap();

    ApiResponse::CustomHTML(status, mime, html)
}

pub fn get_community_info_or_return_validation_error<'r>(
    metadata: &Metadata<'r>,
    community_info: Result<Form<CreateCommunity<'r>>, Errors<'r>>,
) -> Result<CreateCommunity<'r>, ApiResponse> {
    Ok(community_info
        .map_err(|errors| {
            let mut community_name_error: Option<String> = None;
            let mut description_error: Option<String> = None;
            let mut honeypot_error: Option<Toast> = None;

            for error in errors.into_iter() {
                let is_for_name = error.is_for_exactly("community_name");
                let is_for_description = error.is_for_exactly("description");
                let is_for_honeypot = error.is_for_exactly("honeypot");

                if is_for_name {
                    community_name_error = Some(error.kind.to_string());
                }

                if is_for_description {
                    description_error = Some(error.kind.to_string());
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
                Status::UnprocessableEntity,
                community_name_error,
                description_error,
                honeypot_error,
            )
        })?
        .into_inner())
}
