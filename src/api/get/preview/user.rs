use rocket::{get, http::Status};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    helpers::db::DbConn,
    models::{
        api::ApiResponse,
        users::metadata::{User, JWT},
    },
};

#[get("/user?<user_id>")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    _jwt: JWT,
    user_id: i32,
) -> Result<ApiResponse, ApiResponse> {
    let user_info = User::get_by_id(&mut db, &user_id).await?;
    let Some(user_info) = user_info else {
        return Err(ApiResponse::String(Status::NotFound, "User not found."));
    };

    Ok(ApiResponse::Template(Template::render(
        "partials/components/users/img_name_info",
        context! {
            name: user_info.display_name,
            img: user_info.display_image
        },
    )))
}
