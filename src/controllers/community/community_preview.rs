use std::str::FromStr;

use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::{
    helpers::db::DbConn,
    models::{community::schema::CommunityPreview, users::schema::UserJWT, StringUuid},
};

impl CommunityPreview {
    pub async fn new_from_db(
        db: &mut Connection<DbConn>,
        community_uid: &Uuid,
        user_uid: &Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT
                uid,
                owner_uid,
                is_viewer_outsider
                FROM communities c
                LEFT JOIN (
                    SELECT
                    c._id,
                    owner_uid,
                    is_viewer_outsider
                    FROM communities c

                    LEFT JOIN (
                        SELECT uid AS owner_uid, _id
                        FROM users
                        WHERE _id = (
                            SELECT owner_id
                            FROM communities
                            WHERE uid = $1
                        )
                        GROUP BY owner_uid, _id
                    ) owner ON c.owner_id = owner._id

                    LEFT JOIN (
                        SELECT NOT EXISTS (
                            SELECT 1
                            FROM community_memberships
                            WHERE _community_id = (
                                SELECT _id
                                FROM communities
                                WHERE uid = $1
                            ) AND _user_id = (
                                SELECT _id
                                FROM users
                                WHERE uid = $2
                            )
                        ) AS is_viewer_outsider,
                        _community_id
                        FROM community_memberships

                        GROUP BY is_viewer_outsider, _community_id
                    ) viewer ON c._id = viewer._community_id

                    GROUP BY c._id, owner.owner_uid, viewer.is_viewer_outsider
                ) cm ON cm._id = c._id
                WHERE uid = $1;
            "#,
            community_uid,
            user_uid
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(result.map(|r| {
            let is_owner = &r.owner_uid == user_uid;
            let is_viewer_outsider = r
                .is_viewer_outsider
                .map(|b| !is_owner && b)
                .unwrap_or_else(|| !is_owner);

            Self {
                owner_uid: r.owner_uid.to_string(),
                community_uid: r.uid.to_string(),
                is_viewer_outsider,
            }
        }))
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for CommunityPreview {
    type Error = &'a str;

    async fn from_request(request: &'a Request<'_>) -> Outcome<CommunityPreview, Self::Error> {
        let Some(uid) = request.param::<StringUuid>(0) else {
            request.local_cache(|| Some("Please provide a community UID."));

            return Outcome::Forward(Status::NotFound);
        };
        let Ok(StringUuid(community_uid)) = uid else {
            request.local_cache(|| Some("Invalid community UID."));

            return Outcome::Forward(Status::BadRequest);
        };
        let jwt_outcome = request.guard::<UserJWT>().await;

        match jwt_outcome {
            Outcome::Success(jwt) => {
                let Outcome::Success(mut db) = Connection::<DbConn>::from_request(request).await
                else {
                    request.local_cache(|| Some("Failed to connect to the database."));

                    return Outcome::Error((
                        Status::InternalServerError,
                        "Failed to connect to the database.",
                    ));
                };
                // already evaluated at its own guard so we unwrap
                let user_uid = Uuid::from_str(&jwt.uid).unwrap();

                match CommunityPreview::new_from_db(&mut db, &community_uid, &user_uid).await {
                    Ok(Some(community_preview)) => Outcome::Success(community_preview),
                    Ok(None) => Outcome::Forward(Status::NotFound),
                    Err(err) => {
                        eprintln!("Sqlx Error: {:?}", err);
                        request.local_cache(|| Some("Failed to fetch community preview."));

                        return Outcome::Error((
                            Status::InternalServerError,
                            "Failed to fetch community preview.",
                        ));
                    }
                }
            }
            Outcome::Error(err) => Outcome::Error(err),
            Outcome::Forward(status) => return Outcome::Forward(status),
        }
    }
}
