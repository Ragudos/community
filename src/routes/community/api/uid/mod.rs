use rocket::get;

use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;

#[get("/<_>")]
pub fn get<'r>(user: UserJWT) -> Result<ApiResponse, ApiResponse> {
    todo!("Implement me!")
}
