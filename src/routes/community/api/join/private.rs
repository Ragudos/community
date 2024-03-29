use rocket::form::{Errors, Form};
use rocket::http::{Header, Status};
use rocket::post;
use rocket::tokio::sync::broadcast::Sender;
use rocket::State;

use rocket_csrf_token::CsrfToken;

use rocket_db_pools::Connection;

use rocket_dyn_templates::{context, Template};

use sqlx::Acquire;

use crate::controllers::errors::extract_data_or_return_response;
use crate::controllers::format_time_difference;
use crate::helpers::db::DbConn;
use crate::models::community::forms::JoinPrivateCommunity;
use crate::models::community::schema::{Community, CommunityJoinRequest, CommunityMembership};
use crate::models::db::enums::NotificationType;
use crate::models::notifications::RealtimeNotification;
use crate::models::notifications::{Notification, UserNotificationPreference};
use crate::models::users::schema::UserJWT;
use crate::models::Toast;
use crate::responders::{ApiResponse, HeaderCount};

#[post("/private", data = "<form>")]
pub async fn private_join_post<'r, 'v>(
    mut db: Connection<DbConn>,
    user: UserJWT,
    form: Result<Form<JoinPrivateCommunity<'r>>, Errors<'r>>,
    csrf_token: CsrfToken,
    realtime_notification: &State<Sender<RealtimeNotification>>,
) -> Result<ApiResponse, ApiResponse> {
    let form = extract_data_or_return_response(form, "partials/community/join/private_error")?;

    csrf_token.verify(&form.authenticity_token.to_string())?;

    if let Some(is_private) = Community::is_private(&mut db, &form.community_id).await? {
        if !is_private {
            return Err(ApiResponse::Render {
                status: Status::Forbidden,
                template: Some(Template::render(
                    "partials/toast",
                    context! {
                        toast: Toast::error(Some("This community has been made private. Please try refreshing the page and try again.".to_string()))
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
                    toast: Toast::error(Some("This community does not exist.".to_string()))
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
                    toast: Toast::error(Some("You are already a member of this community.".to_string()))
                },
            )),
            headers: None,
        });
    }

    if CommunityJoinRequest::did_user_request_to_join(&mut db, &form.community_id, &user._id)
        .await?
    {
        return Err(ApiResponse::Render {
            status: Status::Forbidden,
            template: Some(Template::render(
                "partials/toast",
                context! {
                    toast: Toast::error(Some("You have already requested to join this community.".to_string()))
                },
            )),
            headers: None,
        });
    }

    // Unwrap because we confirmed that the community exists from the is_private if statement above.
    // We only return Option<> so we don't receive an error when a community doesnt exist.
    let community_name = Community::get_name(&mut db, &form.community_id)
        .await?
        .unwrap();
    let message = format!(
        "{} has requested to join your community <strong>{}</strong>.",
        user.display_name, community_name
    );
    let owner_id = Community::get_owner_id(&mut db, &form.community_id).await?;
    let does_owner_want_notifications =
        UserNotificationPreference::does_user_prefer_notification_type(
            &mut db,
            &owner_id,
            NotificationType::CommunityEntrance,
        )
        .await?;
    let mut tx = db.begin().await?;

    let join_request_id =
        CommunityJoinRequest::create(&mut tx, &form.community_id, &user._id, &message).await?;

    if true {
        let notification = Notification::create(
            &mut tx,
            &owner_id,
            NotificationType::CommunityEntrance,
            &message,
        )
        .await?;

        tx.commit().await?;

        let stringified_time_difference = format_time_difference(notification._created_at);
        // We dont need to handle the error if no one is online to receive it.
        // If the owner is offline, they can simple receive this notif once it loads
        // when they turn online.
        println!("Sending notification to owner: {}", owner_id);
        let _ = realtime_notification.send(RealtimeNotification {
            _recipient_id: owner_id,
            message,
            sent_at: stringified_time_difference,
        });
    } else {
        tx.commit().await?;
    }

    let resource_uri = Header::new(
        "Location",
        format!(
            "/community/{}/settings/community_join_requests/{}",
            form.community_id, join_request_id
        ),
    );

    Ok(ApiResponse::Render {
        status: Status::Created,
        template: Some(Template::render(
            "partials/community/join/private_success",
            context! {
                community_name
            },
        )),
        headers: Some(HeaderCount::One(resource_uri)),
    })
}

#[post("/private", rank = 2)]
pub fn unauthorized_join_private() -> ApiResponse {
    ApiResponse::Status(Status::Unauthorized)
}
