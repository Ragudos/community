use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    form::{Errors, Form},
    http::{CookieJar, Status},
    post, State,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Metadata};
use sqlx::Acquire;

use crate::{
    controllers::errors::{
        bcrypt_error::bcrypt_error_to_api_response,
        register_error::{get_register_data_or_return_validation_errors, render_error},
        serde_error::serde_json_error_to_api_response,
        sqlx_error::sqlx_error_to_api_response,
    },
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
pub async fn api_endpoint<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    register_data: Result<Form<RegisterFormData<'r>>, Errors<'r>>,
    rate_limit: &State<RateLimit>,
    metadata: Metadata<'r>,
) -> Result<ApiResponse, ApiResponse> {
    rate_limit.add_to_limit_or_return(&metadata)?;

    let register_data = get_register_data_or_return_validation_errors(&metadata, register_data)?;

    if UserTable::is_name_taken(&mut db, &register_data.display_name)
        .await
        .map_err(|error| {
            sqlx_error_to_api_response(
                error,
                "Something went wrong. Please try again later.",
                &metadata,
            )
        })?
    {
        return Err(render_error(
            &metadata,
            Status::Conflict,
            Some("Please choose a different username"),
            None,
            None,
            None,
        ));
    }

    let hashed_password = hash(register_data.password, DEFAULT_COST).map_err(|error| {
        bcrypt_error_to_api_response(
            &metadata,
            error,
            "Something went wrong. Please try again later.",
        )
    })?;
    let mut tx = db.begin().await.map_err(|error| {
        sqlx_error_to_api_response(
            error,
            "Something went wrong. Please try again later.",
            &metadata,
        )
    })?;
    let user_uid = UserTable::create(&mut tx, &register_data.display_name)
        .await
        .map_err(|error| {
            sqlx_error_to_api_response(
                error,
                "Something went wrong. Please try again later.",
                &metadata,
            )
        })?;

    UserMetadata::create(&mut tx, &user_uid, None, Some(&register_data.gender), None)
        .await
        .map_err(|error| {
            sqlx_error_to_api_response(
                error,
                "Something went wrong. Please try again later.",
                &metadata,
            )
        })?;
    UserCredentials::create(&mut tx, &user_uid, None, &hashed_password, None, None)
        .await
        .map_err(|error| {
            sqlx_error_to_api_response(
                error,
                "Something went wrong. Please try again later.",
                &metadata,
            )
        })?;

    let jwt = UserJWT {
        uid: user_uid.to_string(),
        display_name: register_data.display_name.to_string(),
        display_image: None,
    };
    let cookie = jwt.to_cookie().map_err(|error| {
        serde_json_error_to_api_response(
            &metadata,
            error,
            Status::UnprocessableEntity,
            "Something went wrong. Please try again later.",
        )
    })?;

    tx.commit().await.map_err(|error| {
        sqlx_error_to_api_response(
            error,
            "Something went wrong. Please try again later.",
            &metadata,
        )
    })?;

    cookie_jar.add_private(cookie);

    let (mime, html) = metadata
        .render(
            "partials/components/register/success",
            context! {
                username: &register_data.display_name,
            },
        )
        .unwrap();

    Ok(ApiResponse::CustomHTML(Status::Ok, mime, html))
}
