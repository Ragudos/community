use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{http::CookieJar, post};
use rocket::form::{Errors, Form};
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;

use crate::community_uri;
use crate::controllers::errors::extract_data_or_return_response;
use crate::helpers::db::DbConn;
use crate::models::community::forms::RequestChangeJoinProcess;
use crate::models::community::schema::Community;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::community::change_join_process::RequestChangeJoinProcessJWT;
use crate::routes::community::change_join_process;

#[post("/request-change-join-process", data = "<form>")]
pub async fn request_change_join_process_endpoint<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    form: Result<Form<RequestChangeJoinProcess<'r>>, Errors<'r>>,
    csrf_token: CsrfToken
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(form, "partials/community/settings/request_change_join_process_error")?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    if !Community::is_user_owner(&mut db, &form.community_id, &user._id)
        .await?
        .unwrap_or(false)
    {
        return Ok(ApiResponse::Status(Status::Forbidden));
    }

    let request_change_join_process_jwt = RequestChangeJoinProcessJWT::new(form.community_id, user._id).to_cookie()?;

    cookie_jar.add_private(request_change_join_process_jwt);

    Ok(ApiResponse::Redirect(Redirect::to(
        community_uri!(change_join_process::change_join_process_page(form.community_id))
    )))
}

#[post("/request-change-join-process", rank = 2)]
pub fn unauthorized_request_change_join_process() -> Status {
    Status::Unauthorized
}
