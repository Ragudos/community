use rocket::get;
use rocket::http::CookieJar;
use rocket::http::Header;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::auth_uri;
use crate::controllers::htmx::IsBoosted;
use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityAbout;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::responders::HeaderCount;
use crate::routes::auth::login;

#[get("/<community_id>/about?<includeheader>")]
pub async fn about_community_page<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    community_id: i64,
    includeheader: Option<bool>,
) -> Result<ApiResponse, ApiResponse> {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let community_about = CommunityAbout::get(&mut db, &community_id, &user._id).await?;

    match community_about {
        Some(community) => {
            let display_name = community.display_name.clone();
            let metadata = SeoMetadata::build()
                .theme(theme)
                .title(&display_name)
                .finalize();
            let headers = Header::new("Cache-Control", "max-age=0, private, must-revalidate");
            let headers2 = Header::new("X-Frame-Options", "deny");

            Ok(ApiResponse::Render {
                status: Status::Ok,
                template: Some(Template::render(
                    "pages/community/about",
                    context! { metadata, user, is_boosted, includeheader, community_id, current_page: "about", community },
                )),
                headers: Some(HeaderCount::Many(vec![headers, headers2])),
            })
        }
        None => {
            let metadata = SeoMetadata::build()
                .theme(theme)
                .title("404 Not Found")
                .finalize();

            Ok(ApiResponse::Render {
                status: Status::Ok,
                template: Some(Template::render(
                    "pages/community/not_found",
                    context! { metadata, user, is_boosted, includeheader, community_id, current_page: "about" },
                )),
                headers: None,
            })
        }
    }
}

#[get("/<community_id>/about", rank = 2)]
pub fn unauthorized_page(community_id: i64) -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::login_page(_))))
}
