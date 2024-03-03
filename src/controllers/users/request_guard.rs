use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    serde::json::from_str,
    Request,
};
use rocket_db_pools::Connection;

use crate::{
    helpers::db::DbConn,
    models::{
        users::metadata::{UserToken, JWT},
        JWT_NAME,
    },
};

use super::db::traits::Token;

#[async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = &'r str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<JWT, Self::Error> {
        let Some(cookie) = request.cookies().get_private(JWT_NAME) else {
            return Outcome::Forward(Status::Unauthorized);
        };

        let stringified_jwt = cookie.value();

        let Ok(jwt) = from_str::<JWT>(stringified_jwt) else {
            // Means that the JWT has probably been tampered with.
            request.cookies().remove_private(JWT_NAME);
            return Outcome::Error((
                Status::BadRequest,
                "Unable to process request. Please try again.",
            ));
        };

        let Outcome::Success(mut db) = Connection::<DbConn>::from_request(request).await else {
            return Outcome::Error((
                Status::InternalServerError,
                "The server is unable to process your request. Please try again later.",
            ));
        };

        let Ok(token_query_result) =
            UserToken::db_select_by_refresh_token(&mut db, &jwt.refresh_token).await
        else {
            return Outcome::Error((
                Status::InternalServerError,
                "The server is unable to process your request. Please try again later.",
            ));
        };

        let Some(token) = token_query_result else {
            // We remove the cookie because that means this JWT is now invalid.
            // Meaning that the user logged in on another device.
            request.cookies().remove_private(JWT_NAME);
            return Outcome::Forward(Status::Unauthorized);
        };

        if token.is_expired() {
            request.cookies().remove_private(JWT_NAME);

            let Err(err) = UserToken::db_delete_by_refresh_token(&mut db, &jwt.refresh_token).await
            else {
                return Outcome::Forward(Status::Unauthorized);
            };

            eprintln!("Error deleting refresh token: {:?}", err);
            return Outcome::Error((
                Status::InternalServerError,
                "The server is unable to process your request. Please try again later.",
            ));
        }

        // Since the JWT is automatically deleted on the browser,
        // when it expires, we don't need to delete it here.
        Outcome::Success(jwt)
    }
}
