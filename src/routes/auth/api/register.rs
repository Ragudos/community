use bcrypt::{hash, DEFAULT_COST};
use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Status};
use rocket::{post, State};
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Metadata};
use sqlx::Acquire;

use crate::community_uri;
use crate::controllers::errors::bcrypt_error::bcrypt_error_to_api_response;
use crate::controllers::errors::register_error::{get_register_data_or_return_validation_errors, render_error};
use crate::controllers::errors::serde_error::serde_json_error_to_api_response;
use crate::controllers::errors::sqlx_error::sqlx_error_to_api_response;
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::helpers::db::DbConn;
use crate::models::rate_limiter::RateLimit;
use crate::models::users::form::RegisterFormData;
use crate::models::users::schema::{UserCredentials, UserJWT, UserMetadata, UserTable};
use crate::models::api::ApiResponse;
use crate::controllers::htmx::IsHTMX;
use crate::routes::community;
use crate::models::query::ListQuery;

/// We do nothing if the user is logged in.
#[post("/register")]
pub fn logged_in(_user: UserJWT, is_htmx: IsHTMX) -> ApiResponse {
    match is_htmx {
        IsHTMX(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(community_uri!(community::page(_)))),
        IsHTMX(false) => ApiResponse::Redirect(Redirect::to(community_uri!(community::page(_)))),
    }
}

#[post("/register", data = "<register_data>", rank = 2)]
pub async fn post<'r>(mut db: Connection<DbConn>, cookie_jar: &CookieJar<'r>, template_metadata: Metadata<'r>, is_htmx: IsHTMX, rate_limiter: &State<RateLimit>, register_data: Result<Form<RegisterFormData<'r>>, Errors<'r>>) -> Result<ApiResponse, ApiResponse> {
    rate_limiter.add_to_limit_or_return(&template_metadata)?;

    let register_data = get_register_data_or_return_validation_errors(&template_metadata, register_data)?;
    let is_name_taken = UserTable::is_name_taken(&mut db, &register_data.display_name)
        .await
        .map_err(|error| sqlx_error_to_api_response(error, None, &template_metadata))?;

    if is_name_taken {
        return Err(render_error(&template_metadata, Status::Conflict, Some("Please choose a different username"), None, None, None));
    }

    let hashed_password = hash(register_data.password, DEFAULT_COST)
        .map_err(|error| bcrypt_error_to_api_response(&template_metadata, error, None))?;
    let mut tx = db.begin().await.map_err(|error| sqlx_error_to_api_response(error, None, &template_metadata))?;
    let user_uid = UserTable::create(&mut tx, &register_data.display_name)
        .await
        .map_err(|error| sqlx_error_to_api_response(error, None, &template_metadata))?;

    UserMetadata::create(&mut tx, &user_uid, None, Some(&register_data.gender), None)
        .await
        .map_err(|error| sqlx_error_to_api_response(error, None, &template_metadata))?;
    UserCredentials::create(&mut tx, &user_uid, None, &hashed_password, None, None)
        .await
        .map_err(|error| sqlx_error_to_api_response(error, None, &template_metadata))?;

    let cookie = UserJWT {
        uid: user_uid.to_string(),
        display_name: register_data.display_name.to_string(),
        display_image: None
    }.to_cookie().map_err(|error| serde_json_error_to_api_response(&template_metadata, error, Status::InternalServerError, None))?;

    tx.commit().await.map_err(|error| sqlx_error_to_api_response(error, None, &template_metadata))?;
    cookie_jar.add(cookie);

    let resource_uri = format!("/user/{}", user_uid);

    match is_htmx {
        IsHTMX(true) => {
            let (mime, html) = template_metadata.render(
                "partials/auth/register_success",
                context! { username: &register_data.display_name }
            ).unwrap();

            Ok(ApiResponse::Created(resource_uri, Some((mime, html))))
        },
        IsHTMX(false) => Ok(ApiResponse::Created(resource_uri, None))
    }
}