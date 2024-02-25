use bcrypt::{hash, DEFAULT_COST};
use rocket::{form::Form, http::{CookieJar, Status}, post, uri};
use rocket_db_pools::Connection;
use sqlx::Acquire;
use time::{Duration, OffsetDateTime};

use crate::{
    api::get::root, controllers::htmx::redirect::HtmxRedirect, helpers::db::DbConn, models::{api::ApiResponse, forms::auth::RegisterFormData, users::metadata::{User, UserCredentials, UserMetadata, UserToken, JWT}}
};

#[post("/register", data = "<register_data>", rank = 2)]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'_>,
    register_data: Form<RegisterFormData<'_>>,
) -> ApiResponse {
    let is_name_taken = User::is_name_taken(&mut db, &register_data.display_name).await;

    match is_name_taken {
        Ok(false) => {
            let hash_result = hash(register_data.password.input, DEFAULT_COST);

            match hash_result {
                Ok(hashed_password) => {
                    let tx_result = db.begin().await;

                    match tx_result {
                        Ok(mut tx) => {
                            let user_result = User::create(&mut tx, &register_data.display_name).await;

                            match user_result {
                                Ok(user) => {
                                    let user_id = &user.id;
                                    let user_metadata_result = UserMetadata::create(&mut tx, user_id, &register_data.gender, true).await;

                                    match user_metadata_result {
                                        Ok(_) => {
                                            let user_credentials_result = UserCredentials::create(&mut tx, user_id, &hashed_password).await;

                                            match user_credentials_result {
                                                Ok(_) => {
                                                    let time_today = OffsetDateTime::now_utc();
                                                    let refresh_token = random_string::generate(64, random_string::charsets::ALPHANUMERIC);
                                                    let store_token_result = UserToken::db_create(&mut tx, user_id, &refresh_token).await;

                                                    match store_token_result {
                                                        Ok(_) => {
                                                             let jwt = JWT::new(
                                                                User {
                                                                    id: user.id,
                                                                    display_name: user.display_name,
                                                                    display_image: user.display_image,
                                                                    created_at: user.created_at,
                                                                },
                                                                time_today.saturating_add(Duration::seconds(3600)),
                                                                time_today,
                                                                refresh_token
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
                                                                            eprintln!("Error: {}", err);
                                                                            ApiResponse::String(Status::InternalServerError, "Something went wrong.")
                                                                        }
                                                                    }
                                                                },
                                                                Err(err) => {
                                                                    let _ = tx.rollback().await;
                                                                    eprintln!("Error: {}", err);
                                                                    return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                                                }
                                                            }
                                                        },
                                                        Err(err) => {
                                                            let _ = tx.rollback().await;
                                                            eprintln!("Error storing refresh token: {:?}", err);
                                                            return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                                        }
                                                    }
                                                },
                                                Err(err) => {
                                                    let _ = tx.rollback().await;
                                                    eprintln!("Error: {}", err);
                                                    return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                                }
                                            }
                                        },
                                        Err(err) => {
                                            let _ = tx.rollback().await;
                                            eprintln!("Error: {}", err);
                                            return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                        }
                                    }
                                },
                                Err(err) => {
                                    let _ = tx.rollback().await;
                                    eprintln!("Error: {}", err);
                                    return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                        }
                    }
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return ApiResponse::String(Status::InternalServerError, "Something went wrong.");
                },
            }
        },
        Ok(true) => {
            return ApiResponse::String(Status::Conflict, "Username already exists.");
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            ApiResponse::String(Status::InternalServerError, "Something went wrong.")
        }
    }
}
