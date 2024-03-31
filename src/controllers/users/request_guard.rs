use rocket::async_trait;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_db_pools::Connection;
use serde_json::from_str;

use crate::helpers::db::DbConn;
use crate::models::users::schema::UserJWT;
use crate::models::JWT_NAME;

#[async_trait]
impl<'r> FromRequest<'r> for UserJWT {
    type Error = &'r str;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<UserJWT, Self::Error> {
        let Some(cookie) = request.cookies().get_private(JWT_NAME) else {
            return Outcome::Forward(Status::Unauthorized);
        };
        let stringified_jwt = cookie.value();
        let Ok(jwt) = from_str::<UserJWT>(stringified_jwt) else {
            // Means that the JWT has probably been tampered with.
            request.cookies().remove_private(JWT_NAME);
            return Outcome::Forward(Status::Unauthorized);
        };
        let Outcome::Success(mut db) =
            Connection::<DbConn>::from_request(request).await
        else {
            request.local_cache(|| Some("Failed to connect to the database."));

            return Outcome::Error((
                Status::InternalServerError,
                "Failed to connect to the database.",
            ));
        };

        if let Ok(is_valid) = jwt.verify(&mut db).await {
            if is_valid {
                return Outcome::Success(jwt);
            } else {
                request.cookies().remove_private(JWT_NAME);
                return Outcome::Forward(Status::Unauthorized);
            }
        } else {
            request.local_cache(|| Some("Failed to verify the JWT."));

            return Outcome::Error((
                Status::InternalServerError,
                "Failed to verify the JWT.",
            ));
        }
    }
}
