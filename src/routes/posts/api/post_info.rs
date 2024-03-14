use rocket::get;

use crate::models::users::schema::UserJWT;

use crate::models::api::ApiResponse;

#[get("/<_>/<_>")]
pub async fn get<'r>(user: UserJWT) -> Result<ApiResponse, ApiResponse> {
    todo!("Get all posts in a community.")
}
