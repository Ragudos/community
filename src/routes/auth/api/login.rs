use bcrypt::verify;
use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket::{post, State};
use rocket_db_pools::Connection;
use rocket_dyn_templates::Metadata;

use crate::community_uri;
use crate::controllers::errors::bcrypt_error::bcrypt_error_to_api_response;
use crate::controllers::errors::login_error::{
    get_login_data_or_return_validation_error, render_error,
};
use crate::controllers::errors::serde_error::serde_json_error_to_api_response;
use crate::controllers::errors::sqlx_error::sqlx_error_to_api_response;
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsHTMX;
use crate::helpers::db::DbConn;
use crate::models::api::ApiResponse;
use crate::models::query::ListQuery;
use crate::models::rate_limiter::RateLimit;
use crate::models::users::form::LoginFormData;
use crate::models::users::schema::{UserCredentials, UserJWT};
use crate::routes::community;

#[post("/login")]
pub fn logged_in(_user: UserJWT, is_htmx: IsHTMX) -> ApiResponse {
    match is_htmx {
        IsHTMX(true) => {
            ApiResponse::HtmxRedirect(HtmxRedirect::to(community_uri!(community::page(_))))
        }
        IsHTMX(false) => ApiResponse::Redirect(Redirect::to(community_uri!(community::page(_)))),
    }
}

#[post("/login", data = "<login_data>", rank = 2)]
pub async fn post<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    template_metadata: Metadata<'r>,
    is_htmx: IsHTMX,
    rate_limiter: &State<RateLimit>,
    login_data: Result<Form<LoginFormData<'r>>, Errors<'r>>,
) -> Result<ApiResponse, ApiResponse> {
    rate_limiter.add_to_limit_or_return(&template_metadata)?;

    let login_data = get_login_data_or_return_validation_error(&template_metadata, login_data)?;
    let Some(password_hash) =
        UserCredentials::get_password_hash_by_name(&mut db, &login_data.display_name)
            .await
            .map_err(|error| sqlx_error_to_api_response(error, None, &template_metadata))?
    else {
        return Err(render_error(
            &template_metadata,
            Status::UnprocessableEntity,
            Some("Invalid credentials"),
            Some("Invalid credentials"),
            None,
        ));
    };
    let did_password_match = verify(login_data.password, &password_hash)
        .map_err(|error| bcrypt_error_to_api_response(&template_metadata, error, None))?;

    if !did_password_match {
        return Err(render_error(
            &template_metadata,
            Status::UnprocessableEntity,
            Some("Invalid credentials"),
            Some("Invalid credentials"),
            None,
        ));
    }

    let cookie = UserJWT::get_by_display_name(&mut db, &login_data.display_name)
        .await
        .map_err(|error| sqlx_error_to_api_response(error, None, &template_metadata))?
        // We unwrap because we know that the user exists from the
        // query for the password hash above
        .unwrap()
        .to_cookie()
        .map_err(|error| {
            serde_json_error_to_api_response(
                &template_metadata,
                error,
                Status::InternalServerError,
                None,
            )
        })?;

    cookie_jar.add_private(cookie);

    match is_htmx {
        IsHTMX(true) => Ok(ApiResponse::HtmxRedirect(HtmxRedirect::to(community_uri!(
            community::page(_)
        )))),
        IsHTMX(false) => Ok(ApiResponse::Redirect(Redirect::to(community_uri!(
            community::page(_)
        )))),
    }
}
