use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    form::Form,
    http::{CookieJar, Status},
    post, State,
};
use rocket_db_pools::Connection;
use sqlx::Acquire;

use crate::{
    api::get::auth::root,
    auth_uri,
    controllers::htmx::redirect::HtmxRedirect,
    helpers::db::DbConn,
    models::{
        api::ApiResponse,
        rate_limiter::RateLimit,
        users::{
            form::RegisterFormData,
            schema::{UserCredentials, UserJWT, UserMetadata, UserTable},
        },
    },
};

#[post("/register", data = "<register_data>", rank = 2)]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'_>,
    register_data: Form<RegisterFormData<'_>>,
    rate_limit: &State<RateLimit>,
) -> Result<ApiResponse, ApiResponse> {
    rate_limit.add_to_limit_or_return(
        "The server is experiencing high loads of requests. Please try again later.",
    )?;

    let is_name_taken = UserTable::is_name_taken(&mut db, &register_data.display_name).await?;

    if is_name_taken {
        return Ok(ApiResponse::String(
            Status::Conflict,
            "Please choose a different username.",
        ));
    }

    let hashed_password = hash(register_data.password, DEFAULT_COST)?;
    let mut tx = db.begin().await?;
    let user_uid = UserTable::create(&mut tx, &register_data.display_name).await?;

    UserMetadata::create(&mut tx, &user_uid, None, Some(&register_data.gender), None).await?;
    UserCredentials::create(&mut tx, &user_uid, None, &hashed_password, None, None).await?;

    let jwt = UserJWT {
        uid: user_uid.to_string(),
        display_name: register_data.display_name.to_string(),
        display_image: None,
    };
    let stringified_jwt = jwt.to_cookie()?;

    tx.commit().await?;

    cookie_jar.add_private(stringified_jwt);
    Ok(ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(
        root::page
    ))))
}
