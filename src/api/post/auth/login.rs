use std::str::FromStr;

use bcrypt::verify;
use rocket::{
    form::Form,
    http::{CookieJar, Status},
    post, State,
};
use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::{
    controllers::htmx::redirect::HtmxRedirect,
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
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'_>,
    login_data: Form<LoginFormData<'_>>,
    rate_limit: &State<RateLimit>,
) -> Result<ApiResponse, ApiResponse> {
    rate_limit.add_to_limit_or_return(
        "The server is experiencing high loads of requests. Please try again later.",
    )?;

    let Ok(user) = UserJWT::get_by_display_name(&mut db, &login_data.display_name).await else {
        return Err(ApiResponse::String(
            Status::BadRequest,
            "Something went wrong. Please try again.",
        ));
    };
    let Some(user) = user else {
        return Err(ApiResponse::String(
            Status::Unauthorized,
            "Invalid credentials",
        ));
    };
    let Ok(uid) = Uuid::from_str(&user.uid) else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Something went wrong.",
        ));
    };
    let Ok(password) = UserCredentials::get_password_hash(&mut db, &uid).await else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Something went wrong.",
        ));
    };
    let Ok(does_password_match) = verify(login_data.password, &password) else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Something went wrong.",
        ));
    };

    if does_password_match == false {
        return Err(ApiResponse::String(
            Status::Unauthorized,
            "Invalid credentials.",
        ));
    }

    let jwt = UserJWT {
        uid: user.uid,
        display_name: user.display_name,
        display_image: user.display_image,
    };
    let Ok(cookie) = jwt.to_cookie() else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Something went wrong.",
        ));
    };

    cookie_jar.add_private(cookie);
    Ok(ApiResponse::HtmxRedirect(HtmxRedirect::to("/homepage")))
}
