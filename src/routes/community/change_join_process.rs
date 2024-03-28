use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{get, http::CookieJar};
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::helpers::db::DbConn;
use crate::models::community::schema::Community;
use crate::responders::ApiResponse;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::routes::community::community_uri;
use crate::routes::community::settings;
use crate::create_request_sensitive_action_jwt;

create_request_sensitive_action_jwt!(RequestChangeJoinProcessJWT, "/community/");

fn sensitive_action_error(theme: Theme, community_id: &i64) -> ApiResponse {
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("500 Internal Server Error")
        .finalize();

    return ApiResponse::Render {
        status: Status::InternalServerError,
        template: Some(Template::render(
            "pages/sensitive_action/error",
            context! {
                message: "An error occurred while processing your request.".to_string(),
                path: community_uri!(settings::community_settings_page(community_id, _, _)).to_string(),
                metadata
            },
        )),
        headers: None,
    };
}

#[get("/<community_id>/change-join-process")]
pub async fn change_join_process_page<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    change_join_process_jwt: Result<RequestChangeJoinProcessJWT, &str>,
    csrf_token: CsrfToken,
    community_id: i64,
) -> Result<ApiResponse, ApiResponse> {
    let theme = Theme::from_cookie_jar(cookie_jar);
    let change_join_process_jwt = change_join_process_jwt.map_err(|error| {
        let metadata = SeoMetadata::build()
            .theme(theme.clone())
            .title("Change Join Process")
            .finalize();

        return ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "pages/sensitive_action/error",
                context! {
                    message: error,
                    path: community_uri!(settings::community_settings_page(community_id, _, _)).to_string(),
                    metadata
                },
            )),
            headers: None,
        };
    })?;

    if change_join_process_jwt.community_id != community_id {
        let metadata = SeoMetadata::build()
            .theme(theme.clone())
            .title("403 Forbidden")
            .finalize();

        return Err(ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "pages/sensitive_action/error",
                context! {
                    message: "You are not authorized to change the join process for this community.".to_string(),
                    path: community_uri!(settings::community_settings_page(community_id, _, _)).to_string(),
                    metadata
                },
            )),
            headers: None,
        });
    }

    let authenticity_token = csrf_token.authenticity_token().map_err(|error| {
        eprintln!("Error: {}", error);
        sensitive_action_error(theme.clone(), &community_id)
    })?;

    let Some(community_name) = Community::get_name(&mut db, &community_id)
        .await
        .map_err(|error| {
            eprintln!("Error: {}", error);
            sensitive_action_error(theme.clone(), &community_id)
        })?
    else {
        let metadata = SeoMetadata::build()
            .theme(theme.clone())
            .title("404 Not Found")
            .finalize();

        return Err(ApiResponse::Render {
            status: Status::NotFound,
            template: Some(Template::render(
                "pages/sensitive_action/error",
                context! {
                    message: "Community not found.".to_string(),
                    path: community_uri!(settings::community_settings_page(community_id, _, _)).to_string(),
                    metadata
                },
            )),
            headers: None,
        });
    };

    let is_private = Community::is_private(&mut db, &community_id).await
        .map_err(|error| {
            eprintln!("Error: {}", error);
            sensitive_action_error(theme.clone(), &community_id)
        })?;

    let metadata = SeoMetadata::build()
        .theme(theme.clone())
        .title("Change Join Process")
        .finalize();

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/community/settings/change_join_process",
            context! {
                community_name,
                authenticity_token,
                metadata,
                is_private
            },
        )),
        headers: None,
    })
}

#[get("/<community_id>/change-join-process", rank = 2)]
pub fn unauthorized_page(community_id: i64) -> Redirect {
    Redirect::to(community_uri!(settings::community_settings_page(community_id, _, _)))
}
