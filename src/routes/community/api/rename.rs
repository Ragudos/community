use rocket::form::{Errors, Form};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{post, put};
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::community_uri;
use crate::controllers::errors::extract_data_or_return_response;
use crate::helpers::db::DbConn;
use crate::models::community::forms::EditDisplayName;
use crate::models::community::schema::Community;
use crate::models::users::schema::UserJWT;
use crate::models::Toast;
use crate::responders::ApiResponse;
use crate::routes::community::settings;

fn handle_sqlx_error(error: sqlx::Error) -> Status {
    eprintln!("{:?}", error);
    Status::InternalServerError
}

#[post("/rename", data = "<form>")]
pub async fn non_htmx_rename_endpoint<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Form<EditDisplayName<'r>>,
    csrf: CsrfToken,
) -> Result<Redirect, Status> {
    if !Community::is_user_owner(&mut db, &form.community_id, &user._id)
        .await
        .map_err(handle_sqlx_error)?
        .unwrap_or(false)
    {
        return Err(Status::Forbidden);
    }

    csrf.verify(&form.authenticity_token.to_string())
        .map_err(|_| Status::Forbidden)?;

    let mut tx = db.begin().await.map_err(handle_sqlx_error)?;

    Community::update_name(&mut tx, &form.community_id, &form.display_name)
        .await
        .map_err(handle_sqlx_error)?;

    tx.commit().await.map_err(handle_sqlx_error)?;

    Ok(Redirect::to(community_uri!(
        settings::community_settings_page(form.community_id, _, _)
    )))
}

#[put("/rename", data = "<form>")]
pub async fn rename_endpoint<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Result<Form<EditDisplayName<'r>>, Errors<'r>>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(
        form,
        "partials/community/settings/rename_error",
    )?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    if !Community::is_user_owner(&mut db, &form.community_id, &user._id)
        .await?
        .unwrap_or(false)
    {
        return Err(ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "partials/toast",
                context! {
                    toast: Toast::error(Some("You are not allowed to perform this action.".to_string()))
                },
            )),
            headers: None,
        });
    }

    let mut tx = db.begin().await?;

    Community::update_name(&mut tx, &form.community_id, &form.display_name)
        .await?;

    tx.commit().await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/community/settings/rename_success",
            context! {
                toast: Toast::success(Some(format!("Community name has been renamed to {}", form.display_name))),
                new_name: form.display_name
            },
        )),
        headers: None,
    })
}
