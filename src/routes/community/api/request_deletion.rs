use rocket::form::{Errors, Form};
use rocket::http::{CookieJar, Status};
use rocket::post;
use rocket::response::Redirect;
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;

use crate::community_uri;
use crate::controllers::errors::extract_data_or_return_response;
use crate::helpers::db::DbConn;
use crate::models::community::forms::RequestDeletion;
use crate::models::community::schema::Community;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::community::delete_community::{self, RequestDeletionJWT};

#[post("/request-deletion", data = "<form>")]
pub async fn request_deletion_endpoint<'r>(
    mut db: Connection<DbConn>,
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    form: Result<Form<RequestDeletion<'r>>, Errors<'r>>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(
        form,
        "partials/community/settings/request_deletion_error",
    )?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    if !Community::is_user_owner(&mut db, &form.community_id, &user._id)
        .await?
        .unwrap_or(false)
    {
        return Ok(ApiResponse::Status(Status::Forbidden));
    }

    let request_deletion_token =
        RequestDeletionJWT::new(form.community_id.clone(), user._id.clone()).to_cookie()?;

    cookie_jar.add_private(request_deletion_token);

    Ok(ApiResponse::Redirect(Redirect::to(community_uri!(
        delete_community::delete_community_page(form.community_id)
    ))))
}

#[post("/request-deletion", rank = 2)]
pub fn unauthorized_request_deletion() -> Status {
    Status::Unauthorized
}
