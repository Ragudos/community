use rocket::get;

use crate::models::query::ListQuery;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;

/// Get all posts in a community.
#[get("/<_>?<list_query..>")]
pub async fn get<'r>(
    user: UserJWT,
    list_query: Option<ListQuery<'r>>,
) -> Result<ApiResponse, ApiResponse> {
    todo!("Get all posts in a community.")
}
