use rocket::http::Status;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::error::RecvError;
use rocket::tokio::sync::broadcast::Sender;
use rocket::{get, FromForm, Shutdown, State};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::helpers::db::DbConn;
use crate::models::notifications::{Notification, RealtimeNotification};
use crate::models::users::schema::UserJWT;
use crate::models::NOTIFICATIONS_LIMIT;
use crate::responders::ApiResponse;

pub mod catchers;
pub mod delete;
pub mod mark_as_read;
pub mod read;

#[derive(FromForm)]
pub struct NotificationFilter {
    pub offset: Option<i64>,
}

#[get("/?<oobswap>&<isfirst>&<filter..>")]
pub async fn notifications(
    mut db: Connection<DbConn>,
    user: UserJWT,
    oobswap: Option<bool>,
    isfirst: Option<bool>,
    filter: Option<NotificationFilter>,
) -> Result<ApiResponse, ApiResponse> {
    let offset = filter
        .as_ref()
        .map_or(0, |f| f.offset.unwrap_or(0));

    if offset.is_negative() {
        return Err(ApiResponse::Status(Status::BadRequest));
    }

    let notifications = Notification::get_all_notifications_of_user(
        &mut db,
        &user._id,
        &NOTIFICATIONS_LIMIT,
        &offset,
    )
    .await?;
    let unread_count = notifications.iter().fold(0, |acc, x| {
        acc + if x.is_read.unwrap_or(false) { 0 } else { 1 }
    });
    let has_unread = unread_count > 0;
    let has_read = notifications.len() > unread_count;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/notifications",
            context! {
                notifications,
                oobswap,
                unread_count,
                isfirst,
                offset,
                has_unread,
                has_read
            },
        )),
        headers: None,
    })
}

/// Responsible for sending the "main" notifications, the one
/// that is displayed in the notifications page/bell icon and sends realtime
/// notifications.
#[get("/sse")]
pub async fn sse_notifications(
    user: UserJWT,
    queue: &State<Sender<RealtimeNotification>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg: RealtimeNotification = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(err)) => {
                        eprintln!("Lagged in notifications EventStream error: {:?}", err);
                        continue;
                    }
                },
                _ = &mut end => break,
            };

            if msg._recipient_id == user._id {
                yield Event::json(&msg);
            }
        }
    }
}
