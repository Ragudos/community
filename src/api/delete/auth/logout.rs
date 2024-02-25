use rocket::{delete, http::{CookieJar, Status}};
use rocket_db_pools::Connection;

use crate::{api::get::auth, auth_uri, controllers::htmx::redirect::HtmxRedirect, helpers::db::DbConn, models::{api::ApiResponse, users::metadata::{UserToken, JWT}}};

#[delete("/logout")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    jwt: JWT,
    cookie_jar: &CookieJar<'_>,
) -> ApiResponse {
    let refresh_token = jwt.refresh_token.clone();
    let parse_result = jwt.to_cookie();

    match parse_result {
        Ok(cookie) =>  {
            if let Err(err) = UserToken::db_delete_by_refresh_token(&mut db, &refresh_token).await {
                eprintln!("Error deleting token: {:?}", err);
                return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
            };

            cookie_jar.remove_private(cookie);
            ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(auth::login::page))) 
        },
        Err(err) => {
            eprintln!("Error parsing JWT: {:?}", err);
            return ApiResponse::String(Status::BadRequest, "Something went wrong.");
        }
    }
}

///  We just redirect since a user can possibly login on another device. If they do, and they dont refresh the page where they were previously logged in,
///  then logged out on that old device, this route will be hit. Refer to {@link JWT} request guard.
#[delete("/logout", rank = 2)]
pub async fn deny_delete_request() -> ApiResponse {
    ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(auth::login::page)))
}

