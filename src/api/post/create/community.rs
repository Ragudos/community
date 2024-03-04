use std::{fs::File, io::Read};

use cloud_storage::{Client, Object};
use rocket::{form::Form, fs::TempFile, http::Status, post, State};
use rocket_db_pools::Connection;
use sqlx::Acquire;

use crate::{
    controllers::recaptcha::verify_token,
    helpers::{db::DbConn, get_environment},
    models::{
        api::ApiResponse,
        community::schema::{Community, CommunityMembership},
        forms::community::CreateCommunity,
        rate_limiter::RateLimit,
        users::metadata::{UserRole, JWT},
    },
};

struct ReturnOfUpload {
    object: Object,
    url: String,
}

#[post("/community", data = "<community_info>")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    jwt: JWT,
    community_info: Form<CreateCommunity<'_>>,
    rate_limit: &State<RateLimit>,
) -> Result<ApiResponse, ApiResponse> {
    println!("{:?}", community_info);

    rate_limit.add_to_limit_or_return(
        "The server is experiencing high loads of requests. Please try again later.",
    )?;

    let recaptcha_result = verify_token(&community_info.recaptcha_token).await?;
    let env = get_environment();

    if recaptcha_result.action != Some("create_community".to_string()) && env != "development" {
        return Err(ApiResponse::String(
            Status::Unauthorized,
            "The captcha taken is not meant for this request.",
        ));
    }

    println!("Recaptcha finished");

    if Community::is_name_taken(&mut db, &community_info.display_name).await? {
        return Err(ApiResponse::String(
            Status::Conflict,
            "Community name is already taken.",
        ));
    }

    println!("Name verified to not be taken");

    let display_image_result = upload_image(
        "aaron_community_bucket",
        "community",
        &community_info.display_image,
    )
    .await?;

    println!("Display image uploaded");

    let cover_image_result = upload_image(
        "aaron_community_bucket",
        "community",
        &community_info.cover_image,
    )
    .await?;

    println!("Cover image uploaded");

    let mut tx = db.begin().await?;

    let Ok(community_id) = Community::store(
        &mut tx,
        &community_info.display_name,
        &display_image_result.url,
        &cover_image_result.url,
        &community_info.description,
        community_info.is_private,
        community_info.category.as_ref(),
        &jwt.token.id,
    )
    .await
    else {
        let res = delete_image("aaron_community_bucket", &display_image_result.object.name).await;

        if let Err(err) = res {
            eprintln!("Failed to delete image: {}", err);
        }

        let res = delete_image("aaron_community_bucket", &cover_image_result.object.name).await;

        if let Err(err) = res {
            eprintln!("Failed to delete image: {}", err);
        }

        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Failed to create community.",
        ));
    };

    println!("Community stored");

    let Ok(_) =
        CommunityMembership::store(&mut tx, &jwt.token.id, &community_id, UserRole::Owner).await
    else {
        let res = delete_image("aaron_community_bucket", &display_image_result.object.name).await;

        if let Err(err) = res {
            eprintln!("Failed to delete image: {}", err);
        }

        let res = delete_image("aaron_community_bucket", &cover_image_result.object.name).await;

        if let Err(err) = res {
            eprintln!("Failed to delete image: {}", err);
        }

        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Failed to create community.",
        ));
    };

    println!("Membership stored");

    let Ok(_) = tx.commit().await else {
        let res = delete_image("aaron_community_bucket", &display_image_result.object.name).await;

        if let Err(err) = res {
            eprintln!("Failed to delete image: {}", err);
        }

        let res = delete_image("aaron_community_bucket", &cover_image_result.object.name).await;

        if let Err(err) = res {
            eprintln!("Failed to delete image: {}", err);
        }

        return Err(ApiResponse::String(
            Status::InternalServerError,
            "Failed to create community.",
        ));
    };

    println!("Transaction committed");

    Ok(ApiResponse::String(Status::Ok, "Oki"))
}

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
}
