use std::str::FromStr;

use rocket::{
    form::{Errors, Form},
    http::Status,
    post, State,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Metadata, Template};
use sqlx::{types::Uuid, Acquire};

use crate::{
    controllers::errors::{
        create_community::{get_community_info_or_return_validation_error, render_error},
        sqlx_error::sqlx_error_to_api_response,
    },
    helpers::db::DbConn,
    models::{
        api::ApiResponse,
        community::{forms::CreateCommunity, schema::Community},
        rate_limiter::RateLimit,
        users::schema::UserJWT,
    },
};

/* struct ReturnOfUpload {
    object: Object,
    url: String,
} */

#[post("/community", data = "<community_info>")]
pub async fn api_endpoint<'r>(
    mut db: Connection<DbConn>,
    jwt: UserJWT,
    community_info: Result<Form<CreateCommunity<'r>>, Errors<'r>>,
    rate_limit: &State<RateLimit>,
    metadata: Metadata<'r>,
) -> Result<ApiResponse, ApiResponse> {
    let community_info = get_community_info_or_return_validation_error(&metadata, community_info)?;

    rate_limit.add_to_limit_or_return(&metadata)?;

    if Community::is_name_taken(&mut db, &community_info.display_name)
        .await
        .map_err(|error| {
            sqlx_error_to_api_response(
                error,
                "Failed to create community. Please try again later",
                &metadata,
            )
        })?
    {
        return Err(render_error(
            &metadata,
            Status::Conflict,
            Some("Please choose a different name".to_string()),
            None,
            None,
        ));
    }

    // This is safe because we've already validated the JWT on the request guard.
    let uid = Uuid::from_str(&jwt.uid).unwrap();
    let mut tx = db.begin().await.map_err(|err| {
        sqlx_error_to_api_response(
            err,
            "Failed to create community. Please try again later",
            &metadata,
        )
    })?;

    // We use tx despite there being only one query because we want to ensure that we
    // are consistent in using transactions in INSERT operations.
    let uid = Community::create(
        &mut tx,
        &community_info.display_name,
        &community_info.description,
        &uid,
    )
    .await
    .map_err(|error| {
        sqlx_error_to_api_response(
            error,
            "Failed to create community. Please try again later",
            &metadata,
        )
    })?;
    tx.commit().await.map_err(|error| {
        sqlx_error_to_api_response(
            error,
            "Failed to create community. Please try again later",
            &metadata,
        )
    })?;

    Ok(ApiResponse::Template(Template::render(
        "partials/components/community/create-community-success",
        context! {
            community_name: &community_info.display_name,
            community_uid: &uid,
            user: jwt
        },
    )))
}

/*
async fn delete_image(bucket_name: &str, file_name: &str) -> Result<(), cloud_storage::Error> {
    let cloud_storage = Client::default();
    cloud_storage
        .object()
        .delete(bucket_name, file_name)
        .await?;

    Ok(())
}

async fn upload_image(
    bucket_name: &str,
    folder_name: &str,
    file: &TempFile<'_>,
) -> Result<ReturnOfUpload, ApiResponse> {
    let Some(content_type) = file.content_type() else {
        return Err(ApiResponse::String(
            Status::BadRequest,
            "Content type is not valid.",
        ));
    };

    let Some(file_name) = file.name() else {
        return Err(ApiResponse::String(
            Status::BadRequest,
            "File name is not valid.",
        ));
    };
    let file_name = file_name.trim();

    let Some(file_path) = file.path() else {
        return Err(ApiResponse::String(
            Status::BadRequest,
            "File path is not valid.",
        ));
    };

    let opened_file = File::open(file_path.to_path_buf())?;
    let mut bytes = Vec::new();

    for byte in opened_file.bytes() {
        match byte {
            Ok(byte) => bytes.push(byte),
            Err(_) => {
                return Err(ApiResponse::String(
                    Status::InternalServerError,
                    "Failed to read file.",
                ))
            }
        }
    }

    let cloud_storage = Client::default();
    let uid = random_string::generate(8, random_string::charsets::ALPHANUMERIC);
    let file_name = format!("{}/{}__split__{}", folder_name, uid, file_name);
    let res = cloud_storage
        .object()
        .create(bucket_name, bytes, &file_name, &content_type.to_string())
        .await?;
    let url = format!(
        "https://storage.googleapis.com/{}/{}",
        bucket_name, file_name
    );

    Ok(ReturnOfUpload { object: res, url })
} */
