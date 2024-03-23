use rocket::{catch, Request};
use rocket_dyn_templates::{context, Template};

use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::{Toast, ToastTypes};

#[catch(422)]
pub fn unprocessable_entity(request: &Request) -> &'static str {
    let error_info: &str = request.local_cache(|| "HAha").as_ref();
    println!("error_info: {:?}", error_info);

    "Please check the information you've entered and try again."
}

#[catch(404)]
pub fn not_found(request: &Request) -> Template {
    let theme = Theme::from_cookie_jar(request.cookies());
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("404 Not Found")
        .finalize();

    Template::render(
        "catchers/404",
        context! {
            metadata,
        },
    )
}

#[catch(500)]
pub fn internal_server_error(request: &Request) -> Template {
    if let Some(is_htmx) = request.headers().get_one("Hx-Toast") {
        // Means the request was made by htmx or the client. So, errors
        // are handled by our toaster.
        if is_htmx == "true" {
            return Template::render(
                "partials/toast",
                context! {
                    toast: Toast {
                        message: "An error occured while processing your request.".to_string(),
                        r#type: Some(ToastTypes::Error),
                    }
                },
            );
        }
    }

    // Else, it's just a url that doesn't exist.

    let theme = Theme::from_cookie_jar(request.cookies());
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("500 Internal Server Error")
        .finalize();

    Template::render(
        "catchers/500",
        context! {
            metadata,
            message: "An error occured while processing your request.",
        },
    )
}
