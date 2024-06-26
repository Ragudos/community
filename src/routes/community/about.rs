use rocket::get;
use rocket::http::{CookieJar, Header, Status};
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::controllers::htmx::IsBoosted;
use crate::helpers::db::DbConn;
use crate::models::community::schema::{CommunityAbout, CommunityJoinRequest};
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::{ApiResponse, HeaderCount};

#[get("/<community_id>/about?<includeheader>")]
pub async fn about_community_page<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    community_id: i64,
    includeheader: Option<bool>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let community_about =
        CommunityAbout::get(&mut db, &community_id, &user._id).await?;
    let authenticity_token = csrf_token.authenticity_token()?;
    match community_about {
        Some(community) => {
            let did_user_request_to_join =
                CommunityJoinRequest::did_user_request_to_join(
                    &mut db,
                    &community_id,
                    &user._id,
                )
                .await?;
            let display_name = community.display_name.clone();
            let metadata = SeoMetadata::build()
                .theme(theme)
                .title(&display_name)
                .finalize();

            Ok(ApiResponse::Render {
                status: Status::Ok,
                template: Some(Template::render(
                    "pages/community/about",
                    context! { did_user_request_to_join, authenticity_token, metadata, user, is_boosted, includeheader, community_id, current_page: "about", community },
                )),
                headers: None,
            })
        }
        None => Err(ApiResponse::Status(Status::Unauthorized)),
    }
}
