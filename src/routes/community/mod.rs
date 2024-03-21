use rocket::get;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::auth_uri;
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsBoosted;
use crate::models::query::ListQuery;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::auth::login;

pub mod about;
pub mod api;
pub mod members;
pub mod settings;

// TODO: Implement to get the uid from the URL
#[get("/<community_uid>?<includeheader>&<list_query..>")]
pub fn page<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    includeheader: Option<bool>,
    list_query: Option<ListQuery<'r>>,
    community_uid: &str,
) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render(
        "pages/community",
        context! { includeheader, metadata, user, is_boosted, list_query, community_uid, current_page: "community" },
    )
}

#[get("/<_..>", rank = 4)]
pub fn logged_out(is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(login::page))),
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(auth_uri!(login::page))),
    }
}
