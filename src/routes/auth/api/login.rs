use bcrypt::verify;
use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket::{post, State};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::controllers::errors::{extract_data_or_return_response, ValidationError};
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsHTMX;
use crate::controllers::rate_limiter::{RateLimiter, RateLimiterTrait};
use crate::discover_uri;
use crate::helpers::db::DbConn;
use crate::models::query::ListQuery;
use crate::models::users::form::LoginFormData;
use crate::models::users::schema::{UserCredentials, UserJWT};
use crate::responders::ApiResponse;
use crate::routes::discover;

#[post("/login")]
pub fn logged_in(_user: UserJWT, is_htmx: IsHTMX) -> ApiResponse {
    match is_htmx {
        IsHTMX(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(discover_uri!(discover::page(
            Some(true),
            _
        )))),
        IsHTMX(false) => {
            ApiResponse::Redirect(Redirect::to(discover_uri!(discover::page(Some(true), _))))
        }
    }
}

#[post("/login", data = "<login_data>", rank = 2)]
pub async fn post<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    rate_limiter: &State<RateLimiter>,
    login_data: Result<Form<LoginFormData>, Errors<'r>>,
) -> Result<ApiResponse, ApiResponse> {
    rate_limiter.add_to_limit_or_return()?;

    let login_data = extract_data_or_return_response(login_data, "partials/auth/login_error")?;

    if let Some(password_struct) =
        UserCredentials::get_password_hash_by_name(&mut db, &login_data.display_name).await?
    {
        if !verify(login_data.password, &password_struct.password_hash)? {
            return Err(ApiResponse::Render {
                status: Status::UnprocessableEntity,
                template: Some(Template::render(
                    "partials/auth/login_error",
                    context! {
                        errors: vec![
                            ValidationError {
                                field: Some("username".to_string()),
                                message: "Invalid credentials".to_string()
                            },
                            ValidationError {
                                field: Some("password".to_string()),
                                message: "Invalid credentials".to_string()
                            }
                        ]
                    },
                )),
                headers: None,
            });
        }
    } else {
        return Err(ApiResponse::Render {
            status: Status::UnprocessableEntity,
            template: Some(Template::render(
                "partials/auth/login_error",
                context! {
                    errors: vec! [
                        ValidationError {
                            field: Some("username".to_string()),
                            message: "Invalid credentials".to_string()
                        },
                        ValidationError {
                            field: Some("password".to_string()),
                            message: "Invalid credentials".to_string()
                        }
                    ]
                },
            )),
            headers: None,
        });
    }

    cookie_jar.add_private(
        UserJWT::get_by_display_name(&mut db, &login_data.display_name)
            .await?
            // We unwrap because we know that the user exists from the
            // query for the password hash above
            .unwrap()
            .to_cookie()?,
    );

    Ok(ApiResponse::Redirect(Redirect::to(discover_uri!(
        discover::page(Some(true), _)
    ))))
}
