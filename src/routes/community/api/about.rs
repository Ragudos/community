use rocket::get;
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityAbout;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;

#[get("/<community_id>/about")]
pub async fn get(
    mut db: Connection<DbConn>,
    user: UserJWT,
    community_id: i64,
) -> Result<ApiResponse, ApiResponse> {
    let community_about = CommunityAbout::get(&mut db, &community_id, &user._id).await?;
    let count = CommunityAbout::foo(&mut db, &community_id).await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/community/about_section",
            context! {
                user,
                community: community_about,
            },
        )),
        headers: None,
    })
}
