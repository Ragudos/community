use rocket::{catch, http::Status, Request};
use rocket_dyn_templates::{context, Template};

use crate::{
    controllers::users::preferences::get_theme_from_cookie,
    models::{api::ApiResponse, seo::metadata::SeoMetadata},
};

#[catch(422)]
pub fn unprocessable_entity(_request: &Request) -> &'static str {
    "Please check the information you've entered and try again."
}

#[catch(404)]
pub fn not_found(request: &Request) -> ApiResponse {
    if let Some(is_htmx) = request.headers().get_one("Hx-Request") {
        // Means the request was made by htmx or the client. So, errors
        // are handled by our toaster.
        if is_htmx == "true" {
            return ApiResponse::String(Status::NotFound, "Resource not found");
        }
    }

    // Else, it's just a url that doesn't exist.

    let theme = get_theme_from_cookie(request.cookies());
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("404 Not Found")
        .finalize();

    ApiResponse::Template(Template::render(
        "catchers/404",
        context! {
            metadata,
        },
    ))
}

#[catch(500)]
pub fn internal_server_error(request: &Request) -> ApiResponse {
    if let Some(is_htmx) = request.headers().get_one("Hx-Request") {
        // Means the request was made by htmx or the client. So, errors
        // are handled by our toaster.
        if is_htmx == "true" {
            return ApiResponse::String(
                Status::InternalServerError,
                "An error occurred while processing your request",
            );
        }
    }

    // Else, it's just a url that doesn't exist.

    let theme = get_theme_from_cookie(request.cookies());
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("500 Internal Server Error")
        .finalize();

    ApiResponse::Template(Template::render(
        "catchers/500",
        context! {
            metadata,
        },
    ))
}
