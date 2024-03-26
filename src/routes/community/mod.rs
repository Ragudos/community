use rocket::get;
use rocket::http::CookieJar;
use rocket::http::Header;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::auth_uri;
use crate::community_uri;
use crate::controllers::htmx::IsBoosted;
use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityPreview;
use crate::models::query::ListQuery;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::responders::HeaderCount;
use crate::routes::auth::login;

pub mod about;
pub mod api;
pub mod members;
pub mod settings;

// TODO: Implement to get the uid from the URL
#[get("/<community_id>?<shouldboost>&<includeheader>&<list_query..>")]
pub async fn page<'r>(
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
    let Some(community_preview) = CommunityPreview::get(&mut db, &community_id, &user._id).await?
    else {
        let metadata = SeoMetadata::build()
            .theme(theme)
            .title("404 Not Found")
            .finalize();
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

    let display_name = community_preview.display_name.clone();
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title(&display_name)
        .finalize();
    let headers = Header::new("Cache-Control", "max-age=0, private, must-revalidate");
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

#[get("/<_..>", rank = 4)]
pub fn logged_out() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::page(Some(true)))))
}
