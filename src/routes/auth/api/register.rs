use bcrypt::{hash, DEFAULT_COST};
use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Header, Status};
use rocket::response::Redirect;
use rocket::{post, State};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::controllers::errors::{extract_data_or_return_response, ValidationError};
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsHTMX;
use crate::controllers::rate_limiter::{RateLimiter, RateLimiterTrait};
use crate::discover_uri;
use crate::helpers::db::DbConn;
use crate::models::query::ListQuery;
use crate::models::users::form::RegisterFormData;
use crate::models::users::schema::{UserCredentials, UserJWT, UserMetadata, UserTable};
use crate::responders::{ApiResponse, HeaderCount};
use crate::routes::discover;

/// We do nothing if the user is logged in.
#[post("/register")]
pub fn logged_in(_user: UserJWT, is_htmx: IsHTMX) -> ApiResponse {
    match is_htmx {
        IsHTMX(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(discover_uri!(discover::discover_page(
            Some(true),
            _
        )))),
        IsHTMX(false) => {
            ApiResponse::Redirect(Redirect::to(discover_uri!(discover::discover_page(Some(true), _))))
        }
    }
}

#[post("/register", data = "<register_data>", rank = 2)]
pub async fn post<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    rate_limiter: &State<RateLimiter>,
    register_data: Result<Form<RegisterFormData>, Errors<'r>>,
    is_htmx: IsHTMX,
) -> Result<ApiResponse, ApiResponse> {
    rate_limiter.add_to_limit_or_return()?;

    let register_data =
        extract_data_or_return_response(register_data, "partials/auth/register_error")?;

    if UserTable::is_name_taken(&mut db, &register_data.display_name).await? {
        return Err(ApiResponse::Render {
            status: Status::UnprocessableEntity,
            template: Some(Template::render(
                "partials/auth/register_error",
                context! { errors: vec![ ValidationError { field: Some("username".to_string()), message: "Please choose a different username".to_string() } ] },
            )),
            headers: None,
        });
    }
    let hashed_password = hash(register_data.password, DEFAULT_COST)?;
    let mut tx = db.begin().await?;
    let user_id = UserTable::create(&mut tx, &register_data.display_name).await?;

    UserMetadata::create(&mut tx, &user_id).await?;
    UserCredentials::create(&mut tx, &user_id, None, &hashed_password, None, None).await?;

    let cookie = UserJWT {
        _id: user_id,
        display_name: register_data.display_name.to_string(),
        display_image: None,
    }
    .to_cookie()?;

    tx.commit().await?;
    cookie_jar.add_private(cookie);

    let resource_uri = format!("/user/{}", user_id);
    let header = Header::new("Location", resource_uri);

    match is_htmx {
        IsHTMX(true) => Ok(ApiResponse::Render {
            status: Status::Created,
            template: Some(Template::render(
                "partials/auth/register_success",
                context! { username: register_data.display_name },
            )),
            headers: Some(HeaderCount::One(header)),
        }),
        IsHTMX(false) => Ok(ApiResponse::Redirect(Redirect::to(discover_uri!(
            discover::discover_page(Some(true), _)
        )))),
    }
}
