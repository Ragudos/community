use rocket::form::{Errors, Form};
use rocket::http::{Header, Status};
use rocket::post;
use rocket::State;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::controllers::errors::{extract_data_or_return_response, ValidationError};
use crate::controllers::rate_limiter::{RateLimiter, RateLimiterTrait};
use crate::helpers::db::DbConn;
use crate::models::community::forms::CreateCommunity;
use crate::models::community::schema::Community;
use crate::models::users::schema::{UserJWT, UserTable};
use crate::models::{Toast, ToastTypes};
use crate::responders::{ApiResponse, HeaderCount};

#[post("/community", data = "<community_info>")]
pub async fn post<'r>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    community_info: Result<Form<CreateCommunity>, Errors<'r>>,
    rate_limiter: &State<RateLimiter>,
) -> Result<ApiResponse, ApiResponse> {
    let community_info =
        extract_data_or_return_response(community_info, "partials/auth/login_error")?;

    rate_limiter.add_to_limit_or_return()?;

    if UserTable::count_of_owned_communities(&mut db, &user._id).await? > 0 {
        return Err(ApiResponse::Render {
            status: Status::from_code(508).unwrap_or(Status::Unauthorized),
            template: Some(Template::render(
                "partials/community/creation_error",
                context! {
                    toast: Toast {
                        message: "You already own a community".to_string(),
                        r#type: Some(ToastTypes::Error)
                    }
                },
            )),
            headers: None,
        });
    }

    if Community::is_name_taken(&mut db, &community_info.display_name).await? {
        return Err(ApiResponse::Render {
            status: Status::Conflict,
            template: Some(Template::render(
                "partials/community/creation_error",
                context! {
                    errors: vec![
                        ValidationError {
                            field: Some("community_name".to_string()),
                            message: "Please choose a different name".to_string()
                        }
                    ]
                },
            )),
            headers: None,
        });
    }

    let mut tx = db.begin().await?;
    let community_id = Community::create(
        &mut tx,
        &community_info.display_name,
        &community_info.description,
        &user._id,
    )
    .await?;

    tx.commit().await?;

    let resource_uri = format!("community/{}/about", community_id);
    let header = Header::new("Location", resource_uri);

    Ok(ApiResponse::Render {
        status: Status::Created,
        template: Some(Template::render(
            "partials/community/creation_success",
            context! {
                display_name: community_info.display_name,
            },
        )),
        headers: Some(HeaderCount::One(header)),
    })
}
