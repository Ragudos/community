use rocket::catch;
use rocket_dyn_templates::{context, Template};

use crate::models::Toast;

#[catch(500)]
pub fn auth_api_internal_server_error() -> Template {
    Template::render(
        "partials/toast",
        context! {
            toast: Toast::error(Some("An internal server error occurred. Please try again later.".to_string()))
        },
    )
}

#[catch(403)]
pub fn forbidden_auth_api() -> Template {
    Template::render(
        "partials/toast",
        context! {
            toast: Toast::error(Some("You are not authorized to perform this action. Perhaps the CSRF-TOKEN was removed. Please try refreshing the page.".to_string()))
        },
    )
}
