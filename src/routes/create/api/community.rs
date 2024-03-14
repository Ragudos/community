use std::str::FromStr;

use rocket::form::{Errors, Form};
use rocket::http::Status;
use rocket::{post, State};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Metadata};
use sqlx::types::Uuid;
use sqlx::Acquire;

use crate::controllers::errors::create_community::{get_community_info_or_return_validation_error, render_error};
use crate::controllers::errors::sqlx_error::sqlx_error_to_api_response;
use crate::controllers::htmx::IsHTMX;
use crate::helpers::db::DbConn;
use crate::models::api::ApiResponse;
use crate::models::community::schema::Community;
use crate::models::rate_limiter::RateLimit;
use crate::models::users::schema::{UserJWT, UserTable};
use crate::models::community::forms::CreateCommunity;

#[post("/community", data = "<community_info>")]
pub async fn post<'r>(mut db: Connection<DbConn>, template_metadata: Metadata<'r>, is_htmx: IsHTMX, user: UserJWT, community_info: Result<Form<CreateCommunity<'r>>, Errors<'r>>, rate_limiter: &State<RateLimit>) -> Result<ApiResponse, ApiResponse> {
    let community_info = get_community_info_or_return_validation_error(&template_metadata, community_info)?;

    rate_limiter.add_to_limit_or_return(&template_metadata)?;

    let user_uid = Uuid::from_str(&user.uid).unwrap();

    if UserTable::does_own_community(&mut db, &user_uid)
        .await
        .map_err(|error| sqlx_error_to_api_response(error, Some("Failed to create community. Please try again later"), &template_metadata))? {
            return Err(render_error(&template_metadata, Status::from_code(508).unwrap_or(Status::Unauthorized), Some("You already own a community".to_string()), None, None));
        }

    if Community::is_name_taken(&mut db, &community_info.display_name)
        .await
        .map_err(|error| sqlx_error_to_api_response(error, Some("Failed to create community. Please try again later"), &template_metadata))? {
            return Err(render_error(&template_metadata, Status::Conflict, Some("Please choose a different name".to_string()), None, None));
        }

    let mut tx = db.begin().await.map_err(|error| sqlx_error_to_api_response(error, Some("Failed to create community. Please try again later"), &template_metadata))?;
    let community_uid = Community::create(
        &mut tx,
        &community_info.display_name,
        &community_info.description,
        &user_uid
    )
    .await
    .map_err(|error| sqlx_error_to_api_response(error, Some("Failed to create community. Please try again later"), &template_metadata))?;

    tx.commit().await.map_err(|error| sqlx_error_to_api_response(error, Some("Failed to create community. Please try again later"), &template_metadata))?;

    let resource_uri = format!("community/{}/about", community_uid);

    match is_htmx {
        IsHTMX(true) => {
            let (mime, html) = template_metadata.render(
                "partials/community/creation_succes",
                context! {
                    resource_uri: resource_uri.clone(),
                    community_name: &community_info.display_name,
                }
            ).unwrap();

            Ok(ApiResponse::Created(resource_uri, Some((mime, html))))
        },
        IsHTMX(false) => Ok(ApiResponse::Created(resource_uri, None))
    }
}
