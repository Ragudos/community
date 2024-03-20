use std::str::FromStr;

use rocket::get;
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::types::Uuid;

use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityPost;
use crate::models::community::schema::CommunityPreview;
use crate::models::users::schema::UserJWT;
use crate::models::{StringUuid, COMMUNITY_POST_LIMIT};
use crate::responders::ApiResponse;

#[get("/<community_uid>?<search>&<offset>")]
pub async fn get<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    search: Option<&str>,
    offset: Option<i64>,
    community_uid: StringUuid,
) -> Result<ApiResponse, ApiResponse> {
    let offset = offset.unwrap_or(0);

    if offset.is_negative() {
        return Ok(ApiResponse::Status(Status::BadRequest));
    }

    let StringUuid(community_uid) = community_uid;
    let user_uid = Uuid::from_str(&user.uid)?;
    let Some(community_preview) =
        CommunityPreview::new_from_db(&mut db, &community_uid, &user_uid).await?
    else {
        return Ok(ApiResponse::Status(Status::Forbidden));
    };

    if community_preview.is_viewer_outsider {
        return Ok(ApiResponse::Status(Status::Forbidden));
    }

    let posts = if let Some(search) = search {
        CommunityPost::get_community_posts_with_query(
            &mut db,
            &community_uid,
            search,
            &offset,
            &COMMUNITY_POST_LIMIT,
        )
        .await?
    } else {
        CommunityPost::get_community_posts(&mut db, &community_uid, &offset, &COMMUNITY_POST_LIMIT)
            .await?
    };

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/community/posts",
            context! { posts, user, search, offset, community_uid: community_uid.to_string() },
        )),
        headers: None,
    })
}
