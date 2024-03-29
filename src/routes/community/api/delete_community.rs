/// Subject to future change.
/// It'd be better to mark a community as
/// to_be_deleted and delete all
/// its relevant data one-by-one using a CRON job
/// to avoid overloading the server since we need to
/// images, videos, etc.
use rocket::delete;
use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Status};
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::controllers::errors::{extract_data_or_return_response, ValidationError};
use crate::helpers::db::DbConn;
use crate::models::community::forms::DeleteCommunity;
use crate::models::community::schema::Community;
use crate::models::users::schema::{UserCredentials, UserJWT};
use crate::models::Toast;
use crate::responders::ApiResponse;
use crate::routes::community::delete_community::RequestDeletionJWT;

#[delete("/delete-community", data = "<form>")]
pub async fn delete_community_endpoint<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    delete_jwt: Result<RequestDeletionJWT, &'r str>,
    form: Result<Form<DeleteCommunity<'r>>, Errors<'r>>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let delete_jwt = delete_jwt.map_err(|_| {
        return ApiResponse::Status(Status::Forbidden);
    })?;
    let form = extract_data_or_return_response(
        form,
        "partials/community/settings/delete_community_error",
    )?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    // We just return a status code since if this is true,
    // the user made the request not from our page.
    // The process for deletion is like this:

    // 1. Request for deletion from /community/<community_id>/settings
    // 2. If approved (the user is the owner of the community), create a cookie
    // containing information about the community and the user, and redirect them to the page
    // to verify if they are truly the ones deleting it by asking for their password.
    // 3. We verify the password in this delete endpoint.
    if delete_jwt.user_id != user._id {
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

        cookie_jar.remove_private(delete_jwt.to_cookie()?);

        return Ok(ApiResponse::Render {
            status: Status::UnprocessableEntity,
            template: Some(Template::render(
                "partials/community/settings/delete_community_error",
                context! {
                    errors: vec![
                        ValidationError {
                            field: Some("user_password".to_string()),
                            message: "The password you entered is incorrect.".to_string(),
                        },
                    ],
                    toast: Toast::warning("We revoked your access to this page to prevent brute-force attacks. Please try again by going back to settings.".to_string()),
                    should_refresh: true
                },
            )),
            headers: None,
        });
    }

    let Some(community_name) = Community::get_name(&mut db, &delete_jwt.community_id).await? else {
        return Ok(ApiResponse::Status(Status::InternalServerError));
    };

    let mut tx = db.begin().await?;

    // For now, we just remove it for good from the database since
    // we haven't implemented media files yet.
    Community::soft_delete(&mut tx, &delete_jwt.community_id).await?;

    tx.commit().await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/community/settings/delete_community_success",
            context! {
                community_name
            },
        )),
        headers: None,
    })
}

#[delete("/delete-community", rank = 2)]
pub fn unauthorized_delete_community() -> ApiResponse {
    ApiResponse::Status(Status::Unauthorized)
}
