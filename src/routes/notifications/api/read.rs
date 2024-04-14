use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::patch;
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use sqlx::Acquire;

use crate::helpers::db::DbConn;
use crate::models::notifications::Notification;
use crate::models::users::schema::UserJWT;
use crate::models::Toast;
use crate::responders::ApiResponse;

#[derive(FromForm)]
pub struct ReadNotificationForm {
    pub notification_id: i64,
}

/// Marks a notification as read AND redirects the user
/// to the notification's URL.
#[patch("/read", data = "<form>")]
pub async fn read_notification_endpoint(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Form<ReadNotificationForm>,
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

    // if the first layer is None, the notification with notifiaction_id doesn't exist.
    let Some(link) = Notification::get_link(&mut db, &notification_id).await?
    else {
        return Err(ApiResponse::Status(Status::NotFound));
    };
    let Some(link) = link else {
        return Err(ApiResponse::Toast(
            Status::InternalServerError,
            Toast::error(Some(
                "Something went wrong. Notification link not found."
                    .to_string(),
            )),
        ));
    };

    let mut tx = db.begin().await?;

    Notification::mark_as_read(&mut tx, &notification_id).await?;

    tx.commit().await?;

    Ok(ApiResponse::Redirect(Redirect::to(link)))
}
