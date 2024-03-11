use crate::{
    helpers::db::DbConn,
    models::{users::schema::UserJWT, JWT_NAME},
};
use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    serde::json::from_str,
    Request,
};
use rocket_db_pools::Connection;

#[async_trait]
impl<'r> FromRequest<'r> for UserJWT {
    type Error = &'r str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<UserJWT, Self::Error> {
        let Some(cookie) = request.cookies().get_private(JWT_NAME) else {
            return Outcome::Forward(Status::Unauthorized);
        };
        let stringified_jwt = cookie.value();
        let Ok(jwt) = from_str::<UserJWT>(stringified_jwt) else {
            // Means that the JWT has probably been tampered with.
            request.cookies().remove_private(JWT_NAME);
            return Outcome::Forward(Status::Unauthorized);
        };
        let Outcome::Success(mut db) = Connection::<DbConn>::from_request(request).await else {
            request.local_cache(|| Some("Failed to connect to the database."));

            return Outcome::Error((
                Status::InternalServerError,
                "Failed to connect to the database.",
            ));
        };

        let Ok(is_valid) = jwt.is_valid(&mut db).await else {
            return Outcome::Error((Status::InternalServerError, "Failed to validate the JWT."));
        };

        if !is_valid {
            request.cookies().remove_private(JWT_NAME);
            return Outcome::Forward(Status::Unauthorized);
        }

        Outcome::Success(jwt)
    }
}
