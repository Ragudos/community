use rocket::form::{Errors, Form};
use rocket::http::Status;
use rocket::post;
use rocket::response::Redirect;
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use sqlx::Acquire;

use crate::community_uri;
use crate::controllers::errors::extract_data_or_return_response;
use crate::helpers::db::DbConn;
use crate::models::community::forms::LeaveCommunity;
use crate::models::community::schema::CommunityMembership;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::community::about;

#[post("/leave-community", data = "<form>")]
pub async fn leave_community_endpoint<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Result<Form<LeaveCommunity<'r>>, Errors<'r>>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(
        form,
        "partials/community/settings/request_leave_error",
    )?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    let mut tx = db.begin().await?;

    // Future: If points and leaderboards are implemented, remove the user's points from the community's leaderboard.
    CommunityMembership::remove_user_from_community(
        &mut tx,
        &form.community_id,
        &user._id,
    )
    .await?;

    tx.commit().await?;

    Ok(ApiResponse::Redirect(Redirect::to(community_uri!(
        about::about_community_page(form.community_id, _)
    ))))
}
