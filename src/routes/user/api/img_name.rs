use rocket::{get, http::Status};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::helpers::db::DbConn;
use crate::models::users::schema::{UserJWT, UserNameAndImage};
use crate::responders::ApiResponse;

#[get("/img_name/<id>")]
pub async fn get(
    mut db: Connection<DbConn>,
    user: UserJWT,
    id: i64,
) -> Result<ApiResponse, ApiResponse> {
    if id == user._id {
        return Ok(ApiResponse::Render {
            status: Status::Ok,
            template: Some(Template::render(
                "partials/user/img_name",
                context! {
                    user
                },
            )),
            headers: None,
        });
    }

    let img_name = UserNameAndImage::get(&mut db, &id).await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/user/img_name",
            context! {
                user: img_name,
            },
        )),
        headers: None,
    })
}
