use rocket::delete;
use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Status};
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::controllers::errors::{
    extract_data_or_return_response, ValidationError,
};
use crate::helpers::db::DbConn;
use crate::models::community::forms::LeaveCommunity;
use crate::models::community::schema::{Community, CommunityMembership};
use crate::models::users::schema::{UserCredentials, UserJWT};
use crate::models::Toast;
use crate::responders::ApiResponse;
use crate::routes::community::leave::RequestLeaveJWT;

#[delete("/leave", data = "<form>")]
pub async fn leave_community_endpoint<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    form: Result<Form<LeaveCommunity<'r>>, Errors<'r>>,
    leave_jwt: Result<RequestLeaveJWT, &'r str>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let leave_jwt = leave_jwt.map_err(|_| {
        return ApiResponse::Status(Status::Forbidden);
    })?;
    let form = extract_data_or_return_response(
        form,
        "partials/community/settings/leave_error",
    )?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    if leave_jwt.user_id != user._id {
        return Ok(ApiResponse::Status(Status::Forbidden));
    }

    let Some(password_struct) =
        UserCredentials::get_password_hash(&mut db, &user._id).await?
    else {
        return Ok(ApiResponse::Status(Status::InternalServerError));
    };

    if !bcrypt::verify(form.user_password, &password_struct.password_hash)? {
        cookie_jar.remove_private(leave_jwt.to_cookie()?);
        let time_to_reload = 3;

        return Ok(ApiResponse::Render {
            status: Status::UnprocessableEntity,
            template: Some(Template::render(
                "partials/community/settings/leave_error",
                context! {
                    errors: vec![
                        ValidationError {
                            field: Some("user_password".to_string()),
                            message: "The password you entered is incorrect.".to_string(),
                        },
                    ],
                    toast: Toast::warning(format!(
                        "We revoked your access to this page. You will be redirected back to settings in {}s.", time_to_reload
                    )),
                    should_refresh: true,
                    time_to_reload
                },
            )),
            headers: None,
        });
    }

    let Some(community_name) =
        Community::get_name(&mut db, &leave_jwt.community_id).await?
    else {
        return Ok(ApiResponse::Status(Status::InternalServerError));
    };

    let mut tx = db.begin().await?;

    // Future: If points and leaderboards are implemented, remove the user's points from the community's leaderboard.
    CommunityMembership::remove_user_from_community(
        &mut tx,
        &leave_jwt.community_id,
        &user._id,
    )
    .await?;

    tx.commit().await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/community/settings/leave_success",
            context! {
                community_name
            },
        )),
        headers: None,
    })
}
