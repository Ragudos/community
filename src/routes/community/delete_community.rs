use rocket::get;
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket_csrf_token::CsrfToken;
use rocket_dyn_templates::{context, Template};
use rocket_db_pools::Connection;

use crate::{create_request_sensitive_action_jwt, discover_uri};
use crate::helpers::db::DbConn;
use crate::models::community::schema::Community;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::query::ListQuery;
use crate::responders::ApiResponse;
use crate::routes::community::community_uri;
use crate::routes::community::settings;
use crate::routes::discover;

create_request_sensitive_action_jwt!(RequestDeletionJWT, "/community/");

#[get("/<community_id>/delete-community")]
pub async fn delete_community_page(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'_>,
    deletion_jwt: Result<RequestDeletionJWT, &str>,
    csrf_token: CsrfToken,
    community_id: i64,
) -> Result<ApiResponse, ApiResponse> {
    let theme = Theme::from_cookie_jar(cookie_jar);
    let deletion_jwt = deletion_jwt.map_err(|error| {
        let metadata = SeoMetadata::build()
            .theme(theme.clone())
            .title("Delete Community")
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

    if deletion_jwt.community_id != community_id {
        let metadata = SeoMetadata::build()
            .theme(theme.clone())
            .title("403 Forbidden")
            .finalize();

        return Err(ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "pages/sensitive_action/error",
                context! {
                    message: "You are not authorized to delete this community.".to_string(),
                    path: community_uri!(settings::community_settings_page(community_id, _, _)).to_string(),
                    metadata
                },
            )),
            headers: None,
        });
    }

    let authenticity_token = csrf_token.authenticity_token().map_err(|error| {
        eprintln!("Error: {}", error);

        let metadata = SeoMetadata::build()
            .theme(theme.clone())
            .title("500 Internal Server Error")
            .finalize();

        return ApiResponse::Render {
            status: Status::InternalServerError,
            template: Some(
                Template::render(
                    "pages/sensitive_action/error",
                    context! {
                        message: "An error occurred while processing your request. Please try again later.".to_string(),
                        path: community_uri!(settings::community_settings_page(community_id, _, _)).to_string(),
                        metadata
                    }
                )
            ),
            headers: None
        };
    })?;

    let Some(community_name) = Community::get_name(&mut db, &community_id).await? else {
        let metadata = SeoMetadata::build()
            .theme(theme.clone())
            .title("404 Not Found")
            .finalize();

        return Err(ApiResponse::Render {
            status: Status::NotFound,
            template: Some(
                Template::render(
                    "pages/sensitive_action/error",
                    context! {
                        message: "We cannot find the community you want to delete.".to_string(),
                        path: discover_uri!(discover::discover_page(Some(true), _)).to_string(),
                        metadata
                    }
                )
            ),
            headers: None
        });
    };

    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("Delete Community")
        .finalize();

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/community/settings/delete_community",
            context! {
                authenticity_token,
                metadata,
                community_name
            },
        )),
        headers: None,
    })
}

#[get("/<community_id>/delete-community", rank = 2)]
pub fn unauthorized_page(community_id: i64) -> Redirect {
    Redirect::to(community_uri!(settings::community_settings_page(community_id, _, _)))
}
