use bcrypt::verify;
use rocket::{
    form::{Errors, Form},
    http::{CookieJar, Status},
    post, State,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::Metadata;

use crate::{
    controllers::{
        errors::{
            bcrypt_error::bcrypt_error_to_api_response,
            login_error::{get_login_data_or_return_validation_error, render_error},
            serde_error::serde_json_error_to_api_response,
            sqlx_error::sqlx_error_to_api_response,
        },
        htmx::redirect::HtmxRedirect,
    },
    helpers::db::DbConn,
    models::{
        api::ApiResponse,
        rate_limiter::RateLimit,
        users::{
            form::LoginFormData,
            schema::{UserCredentials, UserJWT},
        },
    },
};

#[post("/login", data = "<login_data>", rank = 2)]
pub async fn api_endpoint<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    login_data: Result<Form<LoginFormData<'r>>, Errors<'r>>,
    rate_limit: &State<RateLimit>,
    metadata: Metadata<'r>,
) -> Result<ApiResponse, ApiResponse> {
    rate_limit.add_to_limit_or_return(&metadata)?;

    let login_data = get_login_data_or_return_validation_error(&metadata, login_data)?;
    let Some(password_hash) =
        UserCredentials::get_password_hash_by_name(&mut db, &login_data.display_name)
            .await
            .map_err(|error| {
                sqlx_error_to_api_response(
                    error,
                    "Something went wrong. Please try again later.",
                    &metadata,
                )
            })?
    else {
        return Err(render_error(
            &metadata,
            Status::BadRequest,
            Some("Invalid credentials"),
            Some("Invalid credentials"),
            None,
        ));
    };

    if !verify(login_data.password, &password_hash).map_err(|error| {
        bcrypt_error_to_api_response(
            &metadata,
            error,
            "We cannot verify your password. Please try again later.",
        )
    })? {
        return Err(render_error(
            &metadata,
            Status::Unauthorized,
            Some("Invalid credentials"),
            Some("Invalid credentials"),
            None,
        ));
    }

    // We unwrap the Option<UserJWT> since we already verified that the user exists when
    // getting the password.
    let cookie = UserJWT::get_by_display_name(&mut db, &login_data.display_name)
        .await
        .map_err(|error| {
            sqlx_error_to_api_response(
                error,
                "Something went wrong. Please try again later.",
                &metadata,
            )
        })?
        .unwrap()
        .to_cookie()
        .map_err(|error| {
            serde_json_error_to_api_response(
                &metadata,
                error,
                Status::UnprocessableEntity,
                "Something went wrong. Please try again later.",
            )
        })?;

    cookie_jar.add_private(cookie);
    Ok(ApiResponse::HtmxRedirect(HtmxRedirect::to("/homepage")))
}
