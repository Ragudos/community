use std::convert::Infallible;

use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    serde::json::from_str,
    Request,
};
use rocket_db_pools::Connection;
use time::{Duration, OffsetDateTime};

use crate::{
    helpers::db::DbConn,
    models::{
        users::metadata::{UserToken, JWT}, JWT_NAME
    },
};

use super::db::traits::Token;

#[async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<JWT, Self::Error> {
        let cookie = request.cookies().get_private(JWT_NAME).and_then(|cookie| {
            let parsed_jwt = from_str::<JWT>(cookie.value_trimmed());

            match parsed_jwt {
                Ok(jwt) => Some(jwt),
                Err(err) => {
                    eprintln!("Error parsing JWT: {:?}", err);
                    None
                }
            }
        });

        match cookie {
            Some(jwt) => {
                let db = Connection::<DbConn>::from_request(request).await;

                match db {
                    Outcome::Success(mut db) => {
                        // We select by refresh token so that if a user logs in from a different
                        // account,
                        // the old refresh token is invalidated. Therefore, logging this client
                        // out.
                        let token_query_result =
                            UserToken::db_select_by_refresh_token(&mut db, &jwt.refresh_token).await;
                        match token_query_result {
                            Ok(Some(token)) => {
                                if !jwt.is_expired() {
                                    return Outcome::Success(jwt);
                                }

                                if token.is_expired() {
                                    let res =
                                        UserToken::db_delete_by_refresh_token(&mut db, &jwt.refresh_token)
                                            .await;

                                    request.cookies().remove_private(JWT_NAME);

                                    if res.is_err() {
                                        eprintln!(
                                            "Error deleting refresh token: {:?}",
                                            res.err().unwrap()
                                        );
                                        return Outcome::Forward(Status::InternalServerError);
                                    }

                                    return Outcome::Forward(Status::Unauthorized);
                                }

                                let new_jwt = JWT {
                                    token: jwt.token,
                                    expires_in: OffsetDateTime::now_utc()
                                        .saturating_add(Duration::seconds(3600)),
                                    creation_date: OffsetDateTime::now_utc(),
                                    refresh_token: jwt.refresh_token,
                                };

                                if let Ok(cookie) = new_jwt.to_cookie() {
                                    request.cookies().add_private(cookie);
                                } else {
                                    eprintln!("Error creating JWT cookie");
                                    return Outcome::Forward(Status::InternalServerError);
                                }

                                return Outcome::Success(new_jwt);
                            },
                            Ok(None) => {
                                request.cookies().remove_private(JWT_NAME);
                                return Outcome::Forward(Status::Unauthorized);
                            },
                            Err(err) => {
                                request.cookies().remove_private(JWT_NAME);
                                eprintln!("Error querying refresh token: {:?}", err);
                                return Outcome::Forward(Status::InternalServerError);
                            }
                        }
                    }
                    Outcome::Forward(_) => Outcome::Forward(Status::InternalServerError),
                    Outcome::Error(_) => Outcome::Forward(Status::InternalServerError),
                }
            }
            None => {
                request.cookies().remove_private(JWT_NAME);
                Outcome::Forward(Status::Unauthorized)
            },
        }
    }
}
