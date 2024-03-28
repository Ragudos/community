use rocket::get;
use rocket::http::CookieJar;
use rocket::http::Header;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::community_uri;
use crate::controllers::htmx::IsBoosted;
use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityPreview;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::responders::HeaderCount;
use crate::routes::community::about;

#[get("/<community_id>/settings?<includeheader>&<includemainheader>")]
pub async fn community_settings_page<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    includeheader: Option<bool>,
    includemainheader: Option<bool>,
    community_id: i64,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    
    let Some(community_preview) = CommunityPreview::get(&mut db, &community_id, &user._id).await?
    else {
        let metadata = SeoMetadata::build()
            .theme(theme.clone())
            .title("404 Not Found")
            .finalize();
        return Ok(ApiResponse::Render {
            status: Status::NotFound,
            template: Some(Template::render(
                "pages/community/not_found",
                context! { metadata, user, is_boosted, includeheader, includemainheader, community_id },
            )),
            headers: None,
        });
    };

    if !community_preview.is_viewer_a_member.unwrap_or(false)
        && community_preview.owner_id != user._id
    {
        return Ok(ApiResponse::Redirect(Redirect::to(community_uri!(
            about::about_community_page(community_id, includeheader)
        ))));
    }

    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("General")
        .finalize();
    let authenticity_token = csrf_token.authenticity_token()?;
    let headers = Header::new("Cache-Control", "max-age=0, private, must-revalidate");
    let headers2 = Header::new("X-Frame-Options", "deny");

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/community/settings",
            context! {
                metadata,
                user,
                is_boosted,
                includeheader,
                community_id,
                current_page: "settings",
                community: community_preview,
                authenticity_token,
                includemainheader
            },
        )),
        headers: Some(HeaderCount::Many(vec![headers, headers2])),
    })
}

#[get("/<community_id>/settings", rank = 2)]
pub fn unauthorized_page(community_id: i64) -> Redirect {
    Redirect::to(community_uri!(about::about_community_page(community_id, Some(true))))
}
