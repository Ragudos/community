use rocket::get;
use rocket::http::{CookieJar, Status};
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::helpers::db::DbConn;
use crate::models::community::schema::Community;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::responders::ApiResponse;
use crate::create_request_sensitive_action_jwt;

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
    let deletion_jwt = deletion_jwt.map_err(|_| {
        return ApiResponse::Status(Status::Forbidden);
    })?;

    if deletion_jwt.community_id != community_id {
        return Err(ApiResponse::Status(Status::Forbidden));
    }

    let authenticity_token = csrf_token.authenticity_token().map_err(|error| {
        eprintln!("Error: {}", error);
        return ApiResponse::Status(Status::InternalServerError);
    })?;

    let Some(community_name) = Community::get_name(&mut db, &community_id).await? else {
        return Err(ApiResponse::Status(Status::NotFound));
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
