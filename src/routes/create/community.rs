use rocket::get;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::auth_uri;
use crate::routes::auth::login;
use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;

#[get("/community")]
pub fn community_page<'r>(cookie_jar: &CookieJar<'r>, user: UserJWT, is_boosted: IsBoosted) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render(
        "pages/create/community",
        context! { metadata, user, is_boosted },
    )
}

#[get("/community", rank = 2)]
pub fn unauthorized_page() -> Redirect {
    Redirect::to(auth_uri!(login::login_page(Some(true))))
}
