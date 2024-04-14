use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket::{get, FromForm};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use super::about;
use crate::community_uri;
use crate::controllers::htmx::IsBoosted;
use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityPreview;
use crate::models::db::enums::UserRole;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;

#[derive(FromForm)]
pub struct MemberFilter {
    pub t: Option<UserRole>,
}

#[get("/<community_id>/members?<includeheader>&<t..>")]
pub async fn community_members_page<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    includeheader: Option<bool>,
    t: Option<UserRole>,
    community_id: i64,
) -> Result<ApiResponse, ApiResponse> {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let Some(community_preview) =
        CommunityPreview::get(&mut db, &community_id, &user._id).await?
    else {
        return Err(ApiResponse::Status(Status::NotFound));
    };

    if !community_preview
        .is_viewer_a_member
        .unwrap_or(false)
        && community_preview.owner_id != user._id
    {
        return Ok(ApiResponse::Redirect(Redirect::to(community_uri!(
            about::about_community_page(community_id, includeheader)
        ))));
    }

    let display_name = community_preview.display_name.clone();
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title(&display_name)
        .finalize();

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/community/members",
            context! { metadata, user, is_boosted, includeheader, community_id, current_page: "members", community: community_preview },
        )),
        headers: None,
    })
}
