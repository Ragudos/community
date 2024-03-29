use rocket::form::{Errors, Form};
use rocket::http::Status;
use rocket::post;
use rocket::response::Redirect;
use rocket_csrf_token::CsrfToken;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::community_uri;
use crate::controllers::errors::extract_data_or_return_response;
use crate::helpers::db::DbConn;
use crate::models::community::forms::JoinPublicCommunity;
use crate::models::community::schema::{Community, CommunityMembership};
use crate::models::query::ListQuery;
use crate::models::users::schema::UserJWT;
use crate::models::{Toast, ToastTypes};
use crate::responders::ApiResponse;
use crate::routes::community;

#[post("/public", data = "<form>")]
pub async fn public_join_post<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Result<Form<JoinPublicCommunity<'r>>, Errors<'r>>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(form, "partials/community/join/public_error")?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    if let Some(is_private) = Community::is_private(&mut db, &form.community_id).await? {
        if is_private {
            return Err(ApiResponse::Render {
                status: Status::Forbidden,
                template: Some(Template::render(
                    "partials/toast",
                    context! {
                        toast: Toast {
                            message: "This community has been made private. Please try refreshing the page and try again.".to_string(),
                            r#type: Some(ToastTypes::Error)
                        }
                    },
                )),
                headers: None,
            });
        }
    } else {
        return Err(ApiResponse::Render {
            status: Status::NotFound,
            template: Some(Template::render(
                "partials/toast",
                context! {
                    toast: Toast {
                        message: "This community does not exist.".to_string(),
                        r#type: Some(ToastTypes::Error)
                    }
                },
            )),
            headers: None,
        });
    }

    if CommunityMembership::is_user_a_member(&mut db, &form.community_id, &user._id).await? {
        return Err(ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "partials/toast",
                context! {
                    toast: Toast::error(Some( "You are already a member of this community.".to_string(),))
                },
            )),
            headers: None,
        });
    }

    let mut tx = db.begin().await?;

    CommunityMembership::create(&mut tx, &form.community_id, &user._id).await?;

    tx.commit().await?;

    Ok(ApiResponse::Redirect(Redirect::to(community_uri!(
        community::community_page(form.community_id, Some(true), Some(false), _)
    ))))
}

#[post("/public", rank = 2)]
pub fn unauthorized_join_public() -> ApiResponse {
    ApiResponse::Status(Status::Unauthorized)
}
