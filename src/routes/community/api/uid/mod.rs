use rocket::get;

use crate::models::api::ApiResponse;
use crate::models::users::schema::UserJWT;

#[get("/<_>")]
pub fn get<'r>(user: UserJWT) -> Result<ApiResponse, ApiResponse> {
    todo!("Implement me!")
}
