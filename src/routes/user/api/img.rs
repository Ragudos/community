use rocket::get;
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::helpers::db::DbConn;
use crate::models::users::schema::{UserJWT, UserTable};
use crate::responders::ApiResponse;

#[get("/<user_id>/img")]
pub async fn user_img_endpoint(
    mut db: Connection<DbConn>,
    user_id: i64,
    // Check if user is blocked by user_id
    user: UserJWT,
) -> Result<ApiResponse, ApiResponse> {
    let img = UserTable::get_display_image(&mut db, user_id)
        .await
        .map_err(|f| {
            eprintln!("Failed to get user image: {:?}", f);

            ApiResponse::Render {
                status: Status::InternalServerError,
                template: Some(Template::render(
                    "partials/images/error",
                    context! {},
                )),
                headers: None,
            }
        })?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/images/user",
            context! {
                img
            },
        )),
        headers: None,
    })
}
