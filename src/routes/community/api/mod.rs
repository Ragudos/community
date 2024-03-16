use rocket::get;

use crate::models::api::ApiResponse;
use crate::models::query::ListQuery;
use crate::models::users::schema::UserJWT;

pub mod uid;

/// For displaying possible communities to join
#[get("/?<list_query..>")]
pub fn get<'r>(
    user: UserJWT,
    list_query: Option<ListQuery<'r>>,
) -> Result<ApiResponse, ApiResponse> {
    todo!("Implement me!")
}

/// Just a no content for any request made where the first
/// endpoint has forwarded.
#[get("/<_..>", rank = 2)]
pub fn logged_out() -> ApiResponse {
    ApiResponse::NoContent
}
