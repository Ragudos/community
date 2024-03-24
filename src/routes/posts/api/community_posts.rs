use rocket::get;
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityPost;
use crate::models::community::schema::CommunityPreview;
use crate::models::users::schema::UserJWT;
use crate::models::COMMUNITY_POST_LIMIT;
use crate::responders::ApiResponse;

#[get("/<community_id>?<search>&<offset>")]
pub async fn get<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    search: Option<&str>,
    offset: Option<i64>,
    community_id: i64,
) -> Result<ApiResponse, ApiResponse> {
    let offset = offset.unwrap_or(0);

    if offset.is_negative() {
        return Ok(ApiResponse::Status(Status::BadRequest));
    }

    let Some(community_preview) = CommunityPreview::get(&mut db, &community_id, &user._id).await?
    else {
        return Ok(ApiResponse::Status(Status::Forbidden));
    };

    if community_preview.is_viewer_outsider.unwrap_or(true) {
        return Ok(ApiResponse::Status(Status::Forbidden));
    }

    let posts = CommunityPost::get_community_posts(
        &mut db,
        &community_id,
        search,
        &offset,
        &COMMUNITY_POST_LIMIT,
    )
    .await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/community/posts",
            context! { posts, user, search, offset, community_uid: community_id },
        )),
        headers: None,
    })
}
