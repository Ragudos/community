use rocket::get;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::FromForm;
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

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

/// TODO: Community Preview here with community's uid, owner's uid, and whether the user viewing this
/// page is the owner or member of the community or not.
/// Also do the t filter which stands for type, and it can be either `members`, `admins`, `online`.
#[get("/<community_id>/members?<includeheader>&<t..>")]
pub async fn page<'r>(
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
    let metadata = SeoMetadata::build().theme(theme).finalize();
    let Some(community_preview) = CommunityPreview::get(&mut db, &community_id, &user._id).await?
    else {
        return Ok(ApiResponse::Render {
            status: Status::NotFound,
            template: Some(Template::render(
                "pages/community/not_found",
                context! { metadata, user, is_boosted, includeheader, community_id },
            )),
            headers: None,
        });
    };

    if !community_preview.is_viewer_a_member.unwrap_or(false)
        && community_preview.owner_id != user._id
    {
        return Ok(ApiResponse::Redirect(Redirect::to(community_uri!(
            about::page(community_id, includeheader)
        ))));
    }

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/community/members",
            context! { metadata, user, is_boosted, includeheader, community_id, current_page: "members", community: community_preview },
        )),
        headers: None,
    })
}
