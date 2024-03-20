use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::{helpers::db::DbConn, models::community::schema::CommunityPreview};

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
                .unwrap_or(!is_owner);

            Self {
                owner_uid: r.owner_uid.to_string(),
                community_uid: r.uid.to_string(),
                is_viewer_outsider,
            }
        }))
    }
}
