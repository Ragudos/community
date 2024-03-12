use rocket::{get, http::Status};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Metadata, Template};

use crate::{
    helpers::db::DbConn,
    models::{api::ApiResponse, users::schema::UserJWT, StringUuid, Toast, ToastTypes},
};

#[get("/user/<user_id>")]
pub async fn api_endpoint(
    mut db: Connection<DbConn>,
    _jwt: UserJWT,
    user_id: StringUuid,
    metadata: Metadata<'_>,
) -> Result<ApiResponse, ApiResponse> {
    let StringUuid(user_id) = user_id;
    let user_info: Option<UserJWT> = UserJWT::get_by_uid(&mut db, &user_id).await?;
    let Some(user_info) = user_info else {
        let (mime, html) = metadata
            .render(
                "partials/components/toast",
                context! {
                    toast: Toast {
                        message: "User not found.".to_string(),
                        r#type: Some(ToastTypes::Error)
                    }
                },
            )
            .unwrap();

        return Err(ApiResponse::CustomHTML(Status::NotFound, mime, html));
    };

    Ok(ApiResponse::Template(Template::render(
        "partials/components/users/img_name_info",
        context! {
            name: user_info.display_name,
            img: user_info.display_image
        },
    )))
}
