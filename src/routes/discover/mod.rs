use rocket::get;
use rocket::http::CookieJar;
use rocket::http::Header;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::auth_uri;
use crate::controllers::htmx::IsBoosted;
use crate::models::query::ListQuery;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::responders::HeaderCount;
use crate::routes::auth::login;

pub mod api;

// We don't do anything with the query here since
// this will just transfer over the initial query of the URI
// to the HTML markup for HTMX to handle the AJAX side of things.
#[get("/?<isfromauth>&<list_query..>")]
pub fn page<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    isfromauth: Option<bool>,
    list_query: Option<ListQuery<'r>>,
) -> ApiResponse {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("Discover Communities")
        .finalize();
    let headers = Header::new("Cache-Control", "max-age=0, private, must-revalidate");
    let headers2 = Header::new("X-Frame-Options", "deny");

    ApiResponse::Render {
        status: Status::Ok,
        template: Some(
            Template::render(
                "pages/discover",
                context! { metadata, user, is_boosted, isfromauth, list_query },
            )
        ),
        headers: Some(HeaderCount::Many(vec![headers, headers2]))
    }
}

#[get("/<_..>", rank = 3)]
pub fn logged_out() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::page(Some(true)))))
}
