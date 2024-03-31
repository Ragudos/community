use rocket::get;
use rocket::http::{CookieJar, Header, Status};
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::community_uri;
use crate::controllers::htmx::IsBoosted;
use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityPreview;
use crate::models::query::ListQuery;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::{ApiResponse, HeaderCount};

pub mod about;
pub mod api;
pub mod catchers;
pub mod change_join_process;
pub mod delete_community;
pub mod members;
pub mod settings;

// TODO: Implement to get the uid from the URL
#[get("/<community_id>?<shouldboost>&<includeheader>&<list_query..>")]
pub async fn community_page<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    includeheader: Option<bool>,
    list_query: Option<ListQuery<'r>>,
    community_id: i64,
    shouldboost: Option<bool>,
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
    let headers =
        Header::new("Cache-Control", "max-age=0, private, must-revalidate");
    let headers2 = Header::new("X-Frame-Options", "deny");

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/community",
            context! { includeheader, metadata, user, is_boosted, list_query, shouldboost, community_id, current_page: "community", community: community_preview },
        )),
        headers: Some(HeaderCount::Many(vec![headers, headers2])),
    })
}
