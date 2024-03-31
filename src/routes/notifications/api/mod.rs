use rocket::http::Status;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::error::RecvError;
use rocket::tokio::sync::broadcast::Sender;
use rocket::{get, FromForm, FromFormField, Shutdown, State};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

use crate::helpers::db::DbConn;
use crate::models::notifications::{Notification, RealtimeNotification};
use crate::models::users::schema::UserJWT;
use crate::models::NOTIFICATIONS_LIMIT;
use crate::responders::ApiResponse;

pub mod catchers;

#[derive(FromFormField, Serialize, Deserialize)]
pub enum NotificationFilterEnum {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "unread")]
    Unread,
}

#[derive(FromForm)]
pub struct NotificationFilter {
    pub filter: NotificationFilterEnum,
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

    let notifications = if let Some(filter) = &filter {
        match filter {
            NotificationFilter {
                filter: NotificationFilterEnum::All,
                offset: _,
            } => {
                Notification::get_all_notifications_of_user(
                    &mut db,
                    &user._id,
                    &NOTIFICATIONS_LIMIT,
                    &offset,
                )
                .await?
            }
            NotificationFilter {
                filter: NotificationFilterEnum::Read,
                offset: _,
            } => {
                Notification::get_all_read_notifications_of_user(
                    &mut db,
                    &user._id,
                    &NOTIFICATIONS_LIMIT,
                    &offset,
                )
                .await?
            }
            NotificationFilter {
                filter: NotificationFilterEnum::Unread,
                offset: _,
            } => {
                Notification::get_all_unread_notifications_of_user(
                    &mut db,
                    &user._id,
                    &NOTIFICATIONS_LIMIT,
                    &offset,
                )
                .await?
            }
        }
    } else {
        Notification::get_all_notifications_of_user(
            &mut db,
            &user._id,
            &NOTIFICATIONS_LIMIT,
            &offset,
        )
        .await?
    };
    let unread_count = notifications
        .iter()
        .filter(|n| !n.is_read.unwrap_or(false))
        .count();

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "partials/notifications",
            context! {
                notifications,
                oobswap,
                unread_count,
                isfirst,
                filter: filter.map(|f| f.filter),
                offset: offset
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
