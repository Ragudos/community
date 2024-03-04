use bcrypt::verify;
use rocket::{
    form::Form,
    http::{CookieJar, Status},
    post, State,
};
use rocket_db_pools::Connection;
use sqlx::Acquire;
use time::OffsetDateTime;

use crate::{
    controllers::{htmx::redirect::HtmxRedirect, recaptcha::verify_token},
    helpers::{db::DbConn, get_environment},
    models::{
        api::ApiResponse,
        forms::auth::LoginFormData,
        rate_limiter::RateLimit,
        users::metadata::{User, UserCredentials, UserToken, JWT},
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

    let recaptcha_result = verify_token(&login_data.recaptcha_token).await?;
    let env = get_environment();

    if recaptcha_result.action != Some("login".to_string()) && env != "development" {
        return Err(ApiResponse::String(
            Status::Unauthorized,
            "The captcha taken is not meant for this request.",
        ));
    }

    let Ok(user) = User::get_by_display_name(&mut db, &login_data.display_name).await else {
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
    let Ok(Some(password)) = UserCredentials::get_password_by_id(&mut db, &user.id).await else {
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

    let new_refresh_token = random_string::generate(32, random_string::charsets::ALPHANUMERIC);
    let Ok(mut tx) = db.begin().await else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Something went wrong.",
        ));
    };

    let existing_token = UserToken::db_tx_select_by_user_id(&mut tx, &user.id).await?;

    if let Some(_) = existing_token {
        UserToken::db_update_refresh_token(&mut tx, &user.id, &new_refresh_token).await?;
    } else {
        UserToken::db_create(&mut tx, &user.id, &new_refresh_token).await?;
    }

    tx.commit().await?;

    let time_today = OffsetDateTime::now_utc();
    let jwt = JWT::new(user, time_today, new_refresh_token);
    let Ok(cookie) = jwt.to_cookie() else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Something went wrong.",
        ));
    };

    cookie_jar.add_private(cookie);
    Ok(ApiResponse::HtmxRedirect(HtmxRedirect::to("/homepage")))
}
