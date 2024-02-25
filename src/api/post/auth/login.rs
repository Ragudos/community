use bcrypt::verify;
use rocket::{form::Form, http::{CookieJar, Status}, post, uri};
use rocket_db_pools::Connection;
use sqlx::Acquire;
use time::{Duration, OffsetDateTime};

use crate::{api::get::root, controllers::htmx::redirect::HtmxRedirect, helpers::db::DbConn, models::{api::ApiResponse, forms::auth::LoginFormData, users::metadata::{User, UserCredentials, UserToken, JWT}}};

#[post("/login", data = "<login_data>", rank = 2)]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'_>,
    login_data: Form<LoginFormData<'_>>,
) -> ApiResponse {
    let user_result = User::get_by_display_name(&mut db, &login_data.display_name).await;
    
    match user_result {
        Ok(user) => {
            match user {
                Some(user) => {
                    let password_result = UserCredentials::get_password_by_id(&mut db, &user.id).await;

                    match password_result {
                        Ok(Some(hashed_password)) => {
                            let verification_result = verify(login_data.password, &hashed_password);

                            match verification_result {
                                Ok(true) => {
                                    // To check if a user is already logged in on a different
                                    // device.
                                    let refresh_token_result = UserToken::db_select_by_user_id(&mut db, &user.id).await;

                                    match refresh_token_result {
                                        Ok(Some(_refresh_token)) => {
                                            let tx_result = db.begin().await;
                                            
                                            match tx_result {
                                                Ok(mut tx) => {
                                                    let new_token = random_string::generate(64, random_string::charsets::ALPHANUMERIC);
                                                    let update_result = UserToken::db_update_refresh_token(&mut tx, &user.id, &new_token).await;
                                                    match update_result {
                                                        Ok(_) => {
                                                            let time_today = OffsetDateTime::now_utc();
                                                            let jwt = JWT::new(
                                                                user,
                                                                time_today.saturating_add(Duration::seconds(3600)),
                                                                time_today,
                                                                new_token
                                                            );

                                                            let parse_result = jwt.to_cookie();
                                                            match parse_result {
                                                                Ok(cookie) => {
                                                                    let commit_result = tx.commit().await;

                                                                    match commit_result {
                                                                        Ok(_) => {
                                                                            cookie_jar.add_private(cookie);
                                                                            ApiResponse::HtmxRedirect(HtmxRedirect::to(uri!(root::page)))
                                                                        },
                                                                        Err(err) => {
                                                                            eprintln!("Error committing transaction: {:?}", err);
                                                                            ApiResponse::String(rocket::http::Status::InternalServerError, "Something went wrong.")
                                                                        }
                                                                    }
                                                                },
                                                                Err(err) => {
                                                                    let _ = tx.rollback().await;
                                                                    eprintln!("Error parsing JWT: {:?}", err);
                                                                    ApiResponse::String(rocket::http::Status::InternalServerError, "Something went wrong.")
                                                                }
                                                            }
                                                        },
                                                        Err(err) => {
                                                            eprintln!("Error updating refresh token: {:?}", err);
                                                            ApiResponse::String(rocket::http::Status::InternalServerError, "Something went wrong.")
                                                        }
                                                    }
                                                },
                                                Err(err) => {
                                                    eprintln!("Error starting transaction: {:?}", err);
                                                    return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                                }
                                            }
                                        },
                                        Ok(None) => {
                                            let tx_result = db.begin().await;

                                            match tx_result {
                                                Ok(mut tx) => {
                                                    let new_refresh_token = random_string::generate(64, random_string::charsets::ALPHANUMERIC);
                                                    let token_creation_result = UserToken::db_create(&mut tx, &user.id, &new_refresh_token).await;
    
                                                    match token_creation_result {
                                                        Ok(_) => {
                                                            let time_today = OffsetDateTime::now_utc();
                                                            let jwt = JWT::new(
                                                                user,
                                                                time_today.saturating_add(Duration::seconds(3600)),
                                                                time_today,
                                                                new_refresh_token
                                                            );
                                                            let parse_result = jwt.to_cookie();

                                                            match parse_result {
                                                                Ok(cookie) => {
                                                                    let commit_result = tx.commit().await;

                                                                    match commit_result {
                                                                        Ok(_) => {
                                                                            cookie_jar.add_private(cookie);
                                                                            ApiResponse::HtmxRedirect(HtmxRedirect::to(uri!(root::page)))
                                                                        },
                                                                        Err(err) => {
                                                                            eprintln!("Error committing transaction: {:?}", err);
                                                                            return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                                                        }
                                                                    }
                                                                },
                                                                Err(err) => {
                                                                    let _ = tx.rollback().await;
                                                                    eprintln!("Error parsing JWT: {:?}", err);
                                                                    return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                                                }
                                                            }
                                                        },
                                                        Err(err) => {
                                                            let _ = tx.rollback().await;
                                                            eprintln!("Error creating refresh token: {:?}", err);
                                                            return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                                        }
                                                    }
                                                },
                                                Err(err) => {
                                                    eprintln!("Error starting transaction: {:?}", err);
                                                    return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                                }
                                            }
                                        },
                                        Err(err) => {
                                            eprintln!("Error getting refresh token: {:?}", err);
                                            ApiResponse::String(Status::InternalServerError, "Something went wrong.")
                                        }
                                    }
                                },
                                Ok(false) => ApiResponse::String(Status::Unauthorized, "Invalid credentials"),
                                Err(err) => {
                                    eprintln!("Error verifying password: {:?}", err);
                                    return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                }
                            }
                        },
                        Ok(None) => {
                            eprintln!("User exists but has no password.");
                            return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                        },
                        Err(err) => {
                            eprintln!("Error getting password by user id: {:?}", err);
                            return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                        }
                    }
                },
                None => return ApiResponse::String(Status::Unauthorized, "Invalid credentials")
            }
        },
        Err(err) => {
            eprintln!("Error getting user id by display name: {:?}", err);
            return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
        }
    }
}

