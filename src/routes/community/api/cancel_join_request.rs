use rocket::delete;
use rocket::form::{Form, FromForm};
use rocket_db_pools::Connection;
use sqlx::Acquire;

use crate::controllers::htmx::refresh::HtmxRefresh;
use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityJoinRequest;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;

#[derive(FromForm)]
pub struct CancelJoinRequestForm {
    pub community_id: i64,
}

#[delete("/cancel-join-request", data = "<form>")]
pub async fn cancel_join_request_endpoint(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Form<CancelJoinRequestForm>,
) -> Result<ApiResponse, ApiResponse> {
    let community_id = form.community_id;

    let mut tx = db.begin().await?;

    CommunityJoinRequest::delete_pending_join_request(
        &mut tx,
        &community_id,
        &user._id,
    )
    .await?;

    tx.commit().await?;

    Ok(ApiResponse::HtmxRefresh(HtmxRefresh))
}
