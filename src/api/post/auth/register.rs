use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    form::Form,
    http::{CookieJar, Status},
    post, uri,
};
use rocket_db_pools::Connection;
use sqlx::Acquire;
use time::{Duration, OffsetDateTime};

use crate::{
    api::get::root,
    controllers::{htmx::redirect::HtmxRedirect, recaptcha::verify_token},
    helpers::db::DbConn,
    models::{
        api::ApiResponse,
        forms::auth::RegisterFormData,
        users::metadata::{User, UserCredentials, UserMetadata, UserToken, JWT},
    },
};

#[post("/register", data = "<register_data>", rank = 2)]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'_>,
    register_data: Form<RegisterFormData<'_>>,
) -> Result<ApiResponse, ApiResponse> {
    let recaptcha_result = verify_token(&register_data.recaptcha_token).await?;

    if recaptcha_result.action != Some("register".to_string()) {
        return Err(ApiResponse::String(
            Status::Unauthorized,
            "The captcha taken is not meant for this request.",
        ));
    }

    let is_name_taken = User::is_name_taken(&mut db, &register_data.display_name).await?;

    if is_name_taken {
        return Ok(ApiResponse::String(
            Status::Conflict,
            "Please choose a different username.",
        ));
    }

    let hashed_password = hash(register_data.password.input, DEFAULT_COST)?;
    let mut tx = db.begin().await?;
    let user = User::create(&mut tx, &register_data.display_name).await?;

    UserMetadata::create(&mut tx, &user.id, &register_data.gender, true).await?;
    UserCredentials::create(&mut tx, &user.id, &hashed_password).await?;

    let refresh_token = random_string::generate(32, random_string::charsets::ALPHANUMERIC);

    UserToken::db_create(&mut tx, &user.id, &refresh_token).await?;

    let time_today = OffsetDateTime::now_utc();
    let jwt = JWT::new(
        User {
            id: user.id,
            display_name: user.display_name,
            display_image: user.display_image,
            created_at: user.created_at,
        },
        time_today.saturating_add(Duration::seconds(3600)),
        time_today,
        refresh_token,
    );
    let stringified_jwt = jwt.to_cookie()?;

    tx.commit().await?;

    cookie_jar.add_private(stringified_jwt);
    Ok(ApiResponse::HtmxRedirect(HtmxRedirect::to(uri!(
        root::page
    ))))
}
