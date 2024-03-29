use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Status};
use rocket::put;
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::controllers::errors::{extract_data_or_return_response, ValidationError};
use crate::helpers::db::DbConn;
use crate::models::community::forms::ChangeJoinProcessCommunity;
use crate::models::community::schema::{Community, CommunityJoinRequest};
use crate::models::users::schema::{UserCredentials, UserJWT};
use crate::models::Toast;
use crate::responders::ApiResponse;
use crate::routes::community::change_join_process::RequestChangeJoinProcessJWT;

#[put("/change-join-process", data = "<form>")]
pub async fn change_join_process_endpoint<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    change_join_process_jwt: Result<RequestChangeJoinProcessJWT, &'r str>,
    form: Result<Form<ChangeJoinProcessCommunity<'r>>, Errors<'r>>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let change_join_process_jwt = change_join_process_jwt.map_err(|_| {
        return ApiResponse::Status(Status::Forbidden);
    })?;
    let form = extract_data_or_return_response(
        form,
        "partials/community/settings/change_join_process_error",
    )?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    if change_join_process_jwt.user_id != user._id {
        return Ok(ApiResponse::Status(Status::Forbidden));
    }

    let Some(password_struct) = UserCredentials::get_password_hash(&mut db, &user._id).await?
    else {
        return Ok(ApiResponse::Status(Status::InternalServerError));
    };

    if !bcrypt::verify(form.user_password, &password_struct.password_hash)? {
        // We remove the jwt cookie to not let anyone brute force the password.
        // This will make it more cumbersome to troll the community owner if a person
        // gets access to their logged in device.

        cookie_jar.remove_private(change_join_process_jwt.to_cookie()?);
        let time_to_reload = 3;

        return Ok(ApiResponse::Render {
            status: Status::UnprocessableEntity,
            template: Some(Template::render(
                "partials/community/settings/change_join_process_error",
                context! {
                    errors: vec![
                        ValidationError {
                            field: Some("user_password".to_string()),
                            message: "The password you entered is incorrect.".to_string(),
                        },
                    ],
                    toast: Toast::warning(format!(
                        "We revoked your access to this page to prevent brute-force attacks. You will be redirected back to settings in {}s.", time_to_reload
                    )),
                    should_refresh: true,
                    time_to_reload
                },
            )),
            headers: None,
        });
    }

    let Some(community_name) =
        Community::get_name(&mut db, &change_join_process_jwt.community_id).await?
    else {
        return Err(ApiResponse::Status(Status::InternalServerError));
    };
    let mut tx = db.begin().await?;

    let is_private =
        Community::change_join_process(&mut tx, &change_join_process_jwt.community_id).await?;

    // This means that the community is now public and was previously private, so we delete all join requests if any.
    if !is_private {
        CommunityJoinRequest::delete_all_join_requests_of_community(
            &mut tx,
            &change_join_process_jwt.community_id,
        )
        .await?;
    }

    tx.commit().await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/community/settings/change_join_process_success",
            context! {
                community_name,
                community_id: change_join_process_jwt.community_id,
                is_private
            },
        )),
        headers: None,
    })
}

#[put("/change-join-process", rank = 2)]
pub fn change_join_process_unauthorized_endpoint() -> ApiResponse {
    ApiResponse::Status(Status::Forbidden)
}
