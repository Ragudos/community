use rocket::Request;
use rocket::{catch, response::Redirect};
use rocket_dyn_templates::{context, Template};

use crate::auth_uri;
use crate::models::Toast;
use crate::routes::auth::login;

#[catch(401)]
pub fn unauthorized_catcher() -> Redirect {
    Redirect::to(auth_uri!(login::login_page(_)))
}

#[catch(500)]
pub fn internal_server_error(request: &Request<'_>) -> Template {
    let is_toaster = request.headers().get_one("Toaster").map_or(false, |s| s == "true");
    let message = *request.local_cache(||
        "Something went wrong. Please try again later."
    );

    if is_toaster {
        Template::render(
            "partials/toast",
            context! {
                toast: Toast::error(Some(message.to_string()))
            }
        )
    } else {
        Template::render(
            "pages/error",
            context! {
                message
            }
        )
    }
}
