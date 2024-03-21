use rocket::get;
use rocket::http::{Header, Status};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::models::community::schema::CommunityAbout;
use crate::models::StringUuid;
use crate::responders::{ApiResponse, HeaderCount};
use crate::models::users::schema::UserJWT;
use crate::helpers::db::DbConn;

#[get("/<community_uid>/about")]
pub async fn get(
    mut db: Connection<DbConn>,
    user: UserJWT,
    community_uid: StringUuid
) -> Result<ApiResponse, ApiResponse> {
    let StringUuid(community_uid) = community_uid;
    let community_about = CommunityAbout::get(&mut db, &community_uid).await?;
    let header = Header::new("Cache-Control", "public, max-age=3600");

    Ok(
        ApiResponse::Render {
            status: Status::Ok,
            template: Some(
                Template::render(
                    "partials/community/about_section",
                    context! {
                        user,
                        community: community_about
                    }
                )
            ),
            headers: Some(HeaderCount::One(header))
        }
    )
}

