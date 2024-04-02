use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::{patch, put};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use sqlx::Acquire;

use crate::helpers::db::DbConn;
use crate::models::notifications::Notification;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;

#[derive(FromForm)]
pub struct MarkNotificationAsReadForm {
    pub notification_id: i64,
}

#[put("/mark-all-as-read")]
pub async fn mark_all_as_read_endpoint(
    mut db: Connection<DbConn>,
    user: UserJWT,
) -> Result<ApiResponse, ApiResponse> {
    let mut tx = db.begin().await?;

    let notifications = Notification::mark_all_notifications_of_user_as_read(
        &mut tx, &user._id,
    )
    .await?;

    tx.commit().await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/notifications/marked_all_as_read",
            context! { notifications },
        )),
        headers: None,
    })
}

#[patch("/mark-as-read", data = "<form>")]
pub async fn mark_as_read_endpoint(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Form<MarkNotificationAsReadForm>,
) -> Result<ApiResponse, ApiResponse> {
    let notification_id = form.notification_id;

    if !Notification::does_user_own_notification(
        &mut db,
        &user._id,
        &notification_id,
    )
    .await?
    {
        return Err(ApiResponse::Status(Status::Forbidden));
    }

    let mut tx = db.begin().await?;

    let notification =
        Notification::mark_as_read(&mut tx, &notification_id).await?;

    tx.commit().await?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/notifications/marked_as_read",
            context! {
                notification
            },
        )),
        headers: None,
    })
}
