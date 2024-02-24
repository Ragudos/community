use bcrypt::{hash, DEFAULT_COST};
use rocket::{form::Form, http::{CookieJar, Status}, post, uri};
use rocket_db_pools::Connection;
use sqlx::Acquire;
use time::{Duration, OffsetDateTime};

use crate::{
    controllers::htmx::redirect::HtmxRedirect, helpers::db::DbConn, models::{api::ApiResponse, forms::auth::RegisterFormData, users::metadata::{User, UserCredentials, UserMetadata, JWT}},
    api::get::root
};

#[post("/register", data = "<register_data>", rank = 2)]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'_>,
    register_data: Form<RegisterFormData<'_>>,
) -> ApiResponse {
    match User::is_name_taken(&mut db, &register_data.display_name).await {
        Ok(false) => {
            match hash(register_data.password.input, DEFAULT_COST) {
                Ok(hashed_password) => {
                    match db.begin().await {
                        Ok(mut tx) => {
                            match User::create(&mut tx, &register_data.display_name).await {
                                Ok(user) => {
                                    let user_id = &user.id;

                                    match UserMetadata::create(&mut tx, user_id, &register_data.gender, true).await {
                                        Ok(_) => {
                                            match UserCredentials::create(&mut tx, user_id, &hashed_password).await {
                                                Ok(_) => {
                                                    let date_today = OffsetDateTime::now_utc();
                                                    let jwt = JWT::new(
                                                        User {
                                                            id: user.id,
                                                            display_name: user.display_name,
                                                            display_image: user.display_image,
                                                            created_at: user.created_at
                                                        },
                                                        date_today.saturating_add(Duration::seconds(3600)),
                                                        date_today
                                                    );

                                                    match jwt.to_cookie() {
                                                        Ok(cookie) => {
                                                            match tx.commit().await {
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
