use rocket::{get, http::Status};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{helpers::db::DbConn, models::{users::schema::{UserJWT, UserNameAndImage}, StringUuid}, responders::ApiResponse};

#[get("/img_name/<uid>")]
pub async fn get(
    mut db: Connection<DbConn>,
    user: UserJWT,
    uid: StringUuid
) -> Result<ApiResponse, ApiResponse> {
    let StringUuid(uid) = uid;

    if uid.clone().to_string() == user.uid {
        return Ok(
            ApiResponse::Render {
                status: Status::Ok,
                template: Some(
                    Template::render(
                        "partials/user/img_name",
                        context! {
                            user
                        }
                    )
                ),
                headers: None
            }
        );
    }

    let img_name = UserNameAndImage::get(&mut db, &uid).await?;

    Ok(
        ApiResponse::Render {
            status: Status::Ok,
            template: Some(
                Template::render(
                    "partials/user/img_name",
                    context! {
                        user: img_name,
                    }
                )
            ),
            headers: None
        }
    )
}
