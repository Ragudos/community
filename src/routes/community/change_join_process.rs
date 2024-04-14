use rocket::get;
use rocket::http::{CookieJar, Status};
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::create_request_sensitive_action_jwt;
use crate::helpers::db::DbConn;
use crate::models::community::schema::Community;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;

create_request_sensitive_action_jwt!(
    RequestChangeJoinProcessJWT,
    "/community/"
);

#[get("/<community_id>/change-join-process")]
pub async fn change_join_process_page<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    cookie_jar: &CookieJar<'r>,
    change_join_process_jwt: Result<RequestChangeJoinProcessJWT, &str>,
    csrf_token: CsrfToken,
    community_id: i64,
) -> Result<ApiResponse, ApiResponse> {
    let theme = Theme::from_cookie_jar(cookie_jar);
    let change_join_process_jwt = change_join_process_jwt.map_err(|_| {
        return ApiResponse::Status(Status::Forbidden);
    })?;

    if change_join_process_jwt.community_id != community_id
        || change_join_process_jwt.user_id != user._id
    {
        return Err(ApiResponse::Status(Status::Forbidden));
    }

    let authenticity_token =
        csrf_token.authenticity_token().map_err(|error| {
            eprintln!("Error: {}", error);
            return ApiResponse::Status(Status::InternalServerError);
        })?;

    let Some(community_name) = Community::get_name(&mut db, &community_id)
        .await
        .map_err(|error| {
            eprintln!("Error: {}", error);
            return ApiResponse::Status(Status::InternalServerError);
        })?
    else {
        return Err(ApiResponse::Status(Status::NotFound));
    };

    let is_private = Community::is_private(&mut db, &community_id)
        .await
        .map_err(|error| {
            eprintln!("Error: {}", error);
            return ApiResponse::Status(Status::InternalServerError);
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
