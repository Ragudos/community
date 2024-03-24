use rocket_db_pools::Connection;

use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityPreview;

impl CommunityPreview {
    pub async fn get(
        db: &mut Connection<DbConn>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<Option<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
            CommunityPreview,
            r#"
                SELECT
                _id AS community_id,
                owner_id,
                is_viewer_outsider
                FROM communities
                LEFT JOIN (
                    SELECT NOT EXISTS (
                        SELECT 1
                        FROM community_memberships
                        WHERE _community_id = $1
                        AND _user_id = $2
                    ) AS is_viewer_outsider, _community_id
                    FROM community_memberships

                    GROUP BY _community_id, is_viewer_outsider
                ) cm ON _id = cm._community_id
                WHERE _id = $1;
                "#,
            community_id,
            user_id
        )
        .fetch_optional(&mut ***db)
        .await?)
    }
}
