use rocket::form::{Errors, Form};
use rocket::http::Status;
use rocket::post;
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::controllers::errors::extract_data_or_return_response;
use crate::controllers::htmx::refresh::HtmxRefresh;
use crate::helpers::db::DbConn;
use crate::models::community::forms::EditDisplayName;
use crate::models::community::schema::Community;
use crate::models::users::schema::UserJWT;
use crate::models::{Toast, ToastTypes};
use crate::responders::ApiResponse;

#[post("/rename", data = "<form>")]
pub async fn post<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Result<Form<EditDisplayName>, Errors<'r>>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(form, "partials/community/settings/rename_error")?;

    csrf_token.verify(&form.authenticity_token)?;

    if !Community::is_user_owner(&mut db, &form.community_id, &user._id)
        .await?
        .unwrap_or(false)
    {
        return Err(ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "partials/toast",
                context! {
                    toast: Toast {
                        message: "You are not allowed to perform this action.".to_string(),
                        r#type: Some(ToastTypes::Error)
                    }
                },
            )),
            headers: None,
        });
    }

    let mut tx = db.begin().await?;

    Community::update_name(&mut tx, &form.community_id, &form.display_name).await?;

    tx.commit().await?;

    Ok(ApiResponse::HtmxRefresh(HtmxRefresh))
}

#[post("/rename", rank = 2)]
pub fn post_unauthorized() -> Status {
    Status::Unauthorized
}
