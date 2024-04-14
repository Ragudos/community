use rocket::delete;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket_db_pools::Connection;
use sqlx::Acquire;

use crate::helpers::db::DbConn;
use crate::models::notifications::Notification;
use crate::models::users::schema::UserJWT;
use crate::models::Toast;
use crate::responders::ApiResponse;

#[derive(FromForm)]
pub struct DeleteNotificationForm {
    notification_id: i64,
}

#[delete("/delete-all-read")]
pub async fn delete_all_read_notifications_endpoint(
    mut db: Connection<DbConn>,
    user: UserJWT,
) -> Result<ApiResponse, ApiResponse> {
    let mut tx = db.begin().await?;

    Notification::delete_all_read_notifications_of_user(&mut tx, &user._id)
        .await?;

    tx.commit().await?;

    Ok(ApiResponse::Status(Status::NoContent))
}

#[delete("/delete", data = "<form>")]
pub async fn delete_notification_endpoint(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Form<DeleteNotificationForm>,
) -> Result<ApiResponse, ApiResponse> {
    let notification_id = form.notification_id;

    if !Notification::does_exist(&mut db, &notification_id).await? {
        return Err(ApiResponse::Toast(
            Status::NotFound,
            Toast::error(Some(
                "The notification has already been delete".to_string(),
            )),
        ));
    }

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

    Notification::delete(&mut tx, &notification_id).await?;

    tx.commit().await?;

    Ok(ApiResponse::Status(Status::NoContent))
}
