use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Status};
use rocket::post;
use rocket::response::Redirect;
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;

use crate::community_uri;
use crate::controllers::errors::extract_data_or_return_response;
use crate::helpers::db::DbConn;
use crate::models::community::forms::RequestLeave;
use crate::models::community::schema::CommunityMembership;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::community::leave;
use crate::routes::community::leave::RequestLeaveJWT;

#[post("/request-leave", data = "<form>")]
pub async fn request_leave_endpoint<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    form: Result<Form<RequestLeave<'r>>, Errors<'r>>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(
        form,
        "partials/community/settings/request_leave_error",
    )?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    if !CommunityMembership::is_user_a_member(
        &mut db,
        &form.community_id,
        &user._id,
    )
    .await?
    {
        return Ok(ApiResponse::Status(Status::Forbidden));
    }

    let request_leave_token =
        RequestLeaveJWT::new(form.community_id, user._id).to_cookie()?;

    cookie_jar.add_private(request_leave_token);

    Ok(ApiResponse::Redirect(Redirect::to(community_uri!(
        leave::leave_community_page(form.community_id)
    ))))
}
