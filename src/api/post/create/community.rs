use std::str::FromStr;

use rocket::{form::Form, http::Status, post, State};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Metadata};
use sqlx::{types::Uuid, Acquire};

use crate::{
    helpers::db::DbConn,
    models::{api::ApiResponse, community::{forms::CreateCommunity, schema::Community}, rate_limiter::RateLimit, users::schema::UserJWT, Toast, ToastTypes},
};

/* struct ReturnOfUpload {
    object: Object,
    url: String,
} */


#[post("/community", data = "<community_info>")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    jwt: UserJWT,
    community_info: Result<Form<CreateCommunity<'_>>, rocket::form::Errors<'_>>,
    rate_limit: &State<RateLimit>,
    metadata: Metadata<'_>
) -> Result<ApiResponse, ApiResponse> {
    let community_info = community_info.map_err(|errors| {
        let mut community_name_error: Option<String> = None;
        let mut description_error: Option<String> = None; 
        let mut categories_error: Option<String> = None;
        let mut honeypot_error: Option<Toast> = None;

        for error in errors.into_iter() {
            let is_for_name = error.is_for_exactly("community_name");
            let is_for_description = error.is_for_exactly("description");
            let is_for_categories = error.is_for_exactly("category");
            let is_for_honeypot = error.is_for_exactly("honeypot");

            if is_for_name {
                community_name_error = Some(error.kind.to_string());
            }

            if is_for_description {
                description_error = Some(error.kind.to_string());
            }

            if is_for_categories {
                categories_error = Some(error.kind.to_string());
            }

            if is_for_honeypot {
                honeypot_error = Some(Toast {
                    message: error.kind.to_string(),
                    r#type: Some(ToastTypes::Error),
                });
            }
        }

        let (mime, html) = metadata.render(
            "partials/components/community/create-community-errors",
            context! {
                community_name_error,
                description_error,
                categories_error,
                toast: honeypot_error
            }
        ).unwrap();

        ApiResponse::CustomHTML(
            Status::BadRequest,
            mime,
            html
        )
    })?.into_inner();

    rate_limit.add_to_limit_or_return(
        "The server is experiencing high loads of requests. Please try again later.",
    )?;

    if Community::is_name_taken(&mut db, &community_info.display_name).await? {
        return Err(ApiResponse::String(
            Status::Conflict,
            "Community name is already taken.",
        ));
    }

    let mut tx = db.begin().await?;

    let Ok(uid) = Uuid::from_str(&jwt.uid) else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Failed to create community.",
        ));
    };
    Community::create(
        &mut tx,
        &community_info.display_name,
        &community_info.description,
        &uid,
    )
    .await?;

    let Ok(_) = Community::create(
        &mut tx,
        &community_info.display_name,
        &community_info.description,
        &uid,
    )
    .await
    else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Failed to create community.",
        ));
    };

    let Ok(_) = tx.commit().await else {
        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Failed to create community.",
        ));
    };

    println!("Transaction committed");

    Ok(ApiResponse::String(Status::Ok, "Oki"))
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
