use rocket::http::Status;
use rocket::request::Outcome;
use rocket::response::Redirect;
use rocket::{catch, Request};
use rocket_dyn_templates::{context, Template};

use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::auth::login;
use crate::routes::community::{about, settings};
use crate::{auth_uri, community_uri};

#[catch(404)]
pub async fn community_page_not_found_get(
    request: &Request<'_>,
) -> ApiResponse {
    let cookie_jar = request.cookies();
    let metadata = SeoMetadata::build()
        .theme(Theme::from_cookie_jar(cookie_jar))
        .title("404 Page Not Found")
        .finalize();
    let message = *request
        .local_cache(|| "We cannot find this page. Perhaps it's been removed?");
    let user = request.guard::<UserJWT>().await;
    let is_boosted = request
        .headers()
        .get_one("HX-Boosted")
        .map(|str| str.parse::<bool>().unwrap_or_default())
        .unwrap_or(false);
    let includeheader = request
        .query_value::<bool>("includeheader")
        .map(|str| str.unwrap_or(false))
        .unwrap_or(false);

    match user {
        Outcome::Success(user) => ApiResponse::Render {
            status: Status::NotFound,
            template: Some(Template::render(
                "pages/community/error",
                context! { metadata, message, user, is_boosted, includeheader},
            )),
            headers: None,
        },
        _ => {
            ApiResponse::Redirect(Redirect::to(auth_uri!(login::login_page(_))))
        }
    }
}

#[catch(401)]
pub fn community_page_unauthorized_get(request: &Request<'_>) -> Redirect {
    let community_id = request.param::<i64>(0).unwrap().unwrap();
    let is_about_page = request
        .uri()
        .to_string()
        .contains(format!("/community/{}/about", community_id).as_str());
    let is_community_page = request.uri().to_string()
        == "/community".to_string()
        || request.uri().to_string() == "/community/".to_string();
    let is_change_join_process_uri = request.uri().to_string().contains(
        format!("/community/{}/change-join-process", community_id).as_str(),
    );
    let is_delete_community_uri = request
        .uri()
        .to_string()
        .contains(format!("/community/{}/delete", community_id).as_str());

    if is_about_page || is_community_page {
        Redirect::to(auth_uri!(login::login_page(_)))
    } else if is_change_join_process_uri || is_delete_community_uri {
        Redirect::to(community_uri!(settings::community_settings_page(
            community_id,
            _,
            _
        )))
    } else {
        Redirect::to(community_uri!(about::about_community_page(
            community_id,
            Some(true)
        )))
    }
}

#[catch(403)]
pub async fn community_page_forbidden_get(
    request: &Request<'_>,
) -> ApiResponse {
    let cookie_jar = request.cookies();
    let metadata = SeoMetadata::build()
        .theme(Theme::from_cookie_jar(cookie_jar))
        .title("403 Forbidden")
        .finalize();
    let message =
        *request.local_cache(|| "You are not allowed to access this page.");
    let user = request.guard::<UserJWT>().await;
    let is_boosted = request
        .headers()
        .get_one("HX-Boosted")
        .map(|str| str.parse::<bool>().unwrap_or_default())
        .unwrap_or(false);
    let includeheader = request
        .query_value::<bool>("includeheader")
        .map(|str| str.unwrap_or(false))
        .unwrap_or(false);

    match user {
        Outcome::Success(user) => ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "pages/community/error",
                context! { metadata, message, user, is_boosted, includeheader},
            )),
            headers: None,
        },
        _ => {
            ApiResponse::Redirect(Redirect::to(auth_uri!(login::login_page(_))))
        }
    }
}

#[catch(500)]
pub async fn community_page_internal_server_error_get(
    request: &Request<'_>,
) -> ApiResponse {
    let cookie_jar = request.cookies();
    let metadata = SeoMetadata::build()
        .theme(Theme::from_cookie_jar(cookie_jar))
        .title("500 Internal Server Error")
        .finalize();
    let message = *request
        .local_cache(|| "Something went wrong. Please try again later.");
    let user = request.guard::<UserJWT>().await;
    let is_boosted = request
        .headers()
        .get_one("HX-Boosted")
        .map(|str| str.parse::<bool>().unwrap_or_default())
        .unwrap_or(false);
    let includeheader = request
        .query_value::<bool>("includeheader")
        .map(|str| str.unwrap_or(false))
        .unwrap_or(false);

    match user {
        Outcome::Success(user) => ApiResponse::Render {
            status: Status::InternalServerError,
            template: Some(Template::render(
                "pages/community/error",
                context! { metadata, message, user, is_boosted, includeheader},
            )),
            headers: None,
        },
        _ => {
            ApiResponse::Redirect(Redirect::to(auth_uri!(login::login_page(_))))
        }
    }
}
