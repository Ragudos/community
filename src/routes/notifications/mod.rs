use rocket::get;
use rocket::http::Status;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::error::RecvError;
use rocket::tokio::sync::broadcast::Sender;
use rocket::{Shutdown, State};

use crate::models::{notifications::RealtimeNotification, users::schema::UserJWT};

/// Responsible for sending the "main" notifications, the one
/// that is displayed in the notifications page/bell icon.
#[get("/")]
pub async fn main_notifications(
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
                println!("Sending notification to user");
                yield Event::json(&msg);
            }
        }
    }
}

#[get("/", rank = 2)]
pub fn main_notifications_unauthorized() -> Status {
    Status::Unauthorized
}
