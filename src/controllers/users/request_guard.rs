use std::convert::Infallible;

use rocket::{async_trait, http::Status, request::{FromRequest, Outcome}, serde::json::from_str, Request};
use rocket_db_pools::Connection;
use time::{Duration, OffsetDateTime};

use crate::{helpers::db::DbConn, models::users::metadata::{User, UserToken, JWT}};

use super::db::traits::Token;

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, Self::Error> {
        let cookie = request.cookies()
            .get_private("Community__jwt")
            .and_then(|cookie| {
                let parsed_jwt = from_str::<JWT>(cookie.value());

                match parsed_jwt {
                    Ok(jwt) => Some(jwt),
                    Err(err) => {
                        eprintln!("Error parsing JWT: {:?}", err);
                        None
                    },
                }
            });
    
        match cookie {
            Some(jwt) => {
                if !jwt.is_expired() {
                    return Outcome::Success(jwt.token);
                }

                let db = Connection::<DbConn>::from_request(request).await;

                match db {
                    Outcome::Success(mut db) => {
                        let token_query_result = UserToken::db_select_by_user_id(&mut db, jwt.token.id).await;

                        match token_query_result {
                            Ok(token_option) => {
                                match token_option {
                                    Some(token) => {
                                        if token.is_expired() {
                                            let res = UserToken::db_delete_by_user_id(&mut db, jwt.token.id).await;

                                            if res.is_err() {
                                                eprintln!("Error deleting refresh token: {:?}", res.err().unwrap());
                                                return Outcome::Forward(Status::InternalServerError);
                                            }

                                            request.cookies().remove_private("Community__jwt");
                                            return Outcome::Forward(Status::Unauthorized);
                                        }

                                        let new_jwt = JWT {
                                            token: jwt.token.clone(),
                                            expires_in: OffsetDateTime::now_utc().saturating_add(Duration::seconds(3600)),
                                            creation_date: OffsetDateTime::now_utc(),
                                        };

                                        match new_jwt.to_cookie() {
                                            Ok(cookie) => {
                                                request.cookies().add_private(cookie);
                                                return Outcome::Success(jwt.token);
                                            },
                                            Err(err) => {
                                                eprintln!("Error creating JWT cookie: {:?}", err);
                                                return Outcome::Forward(Status::InternalServerError);
                                            }
                                        }
                                    },
                                    None => {
                                        request.cookies().remove_private("Community__jwt");
                                        return Outcome::Forward(Status::Unauthorized);
                                    }
                                }
                            },
                            Err(err) => {
                                eprintln!("Error querying refresh token: {:?}", err);
                                return Outcome::Forward(Status::InternalServerError);
                            }
                        }
                    },
                    Outcome::Forward(_) => Outcome::Forward(Status::InternalServerError),
                    Outcome::Error(_) => Outcome::Forward(Status::InternalServerError),
                }
            },
            None => Outcome::Forward(Status::InternalServerError),
        }
    }
}