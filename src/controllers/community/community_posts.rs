use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::{
    helpers::db::DbConn,
    models::{community::schema::CommunityPost, db::enums::CommunityCategory},
};

impl CommunityPost {
    pub async fn get_community_posts_with_query(
        db: &mut Connection<DbConn>,
        community_uid: &Uuid,
        query: &str,
        offset: &i64,
        limit: &i64,
    ) -> Result<Vec<CommunityPost>, sqlx::Error> {
        let posts = sqlx::query_as!(
            CommunityPost,
            r#"
            SELECT
            is_pinned,
            caption,
            content,
            uid,
            links,
            images,
            videos,
            owner_uid,
            community_uid
            FROM posts p
            LEFT JOIN (
                SELECT owner_id,
                p2._id,
                community_uid,
                owner_uid
                FROM posts p2

                LEFT JOIN (
                    SELECT _id, uid AS owner_uid
                    FROM users

                    GROUP BY _id, uid
                ) u ON u._id = p2.owner_id

                LEFT JOIN (
                    SELECT uid AS community_uid
                    FROM communities

                    GROUP BY community_uid
                ) c ON c.community_uid = $1
                
                GROUP BY p2._id, community_uid, owner_uid
            ) p2 ON p2._id = p._id
            WHERE p._id = (
                SELECT _post_id
                FROM community_posts
                WHERE _community_id = (
                    SELECT _id
                    FROM communities
                    WHERE uid = $1
                )
            ) AND (
                similarity(caption, $2) > 0.1
                OR similarity((
                    SELECT display_name
                    FROM users
                    WHERE _id = p2.owner_id
                ), $2) > 0.1
            )
            ORDER BY _created_at DESC, is_pinned 
            OFFSET $3 LIMIT $4;
            "#,
            community_uid,
            query,
            offset * limit,
            limit
        )
        .fetch_all(&mut ***db)
        .await?;

        Ok(posts)
    }

    pub async fn get_community_posts(
        db: &mut Connection<DbConn>,
        community_uid: &Uuid,
        offset: &i64,
        limit: &i64,
    ) -> Result<Vec<CommunityPost>, sqlx::Error> {
        let posts = sqlx::query_as!(
            CommunityPost,
            r#"
            SELECT
            is_pinned,
            caption,
            content,
            uid,
            links,
            images,
            videos,
            owner_uid,
            community_uid
            FROM posts p
            LEFT JOIN (
                SELECT owner_id,
                p2._id,
                community_uid,
                owner_uid
                FROM posts p2

                LEFT JOIN (
                    SELECT _id, uid AS owner_uid
                    FROM users

                    GROUP BY _id, uid
                ) u ON u._id = p2.owner_id

                LEFT JOIN (
                    SELECT uid AS community_uid
                    FROM communities

                    GROUP BY community_uid
                ) c ON c.community_uid = $1
                
                GROUP BY p2._id, community_uid, owner_uid
            ) p2 ON p2._id = p._id
            WHERE p._id = (
                SELECT _post_id
                FROM community_posts
                WHERE _community_id = (
                    SELECT _id
                    FROM communities
                    WHERE uid = $1
                )
            ) ORDER BY _created_at DESC, is_pinned 
            OFFSET $2 LIMIT $3;
            "#,
            community_uid,
            offset * limit,
            limit
        )
        .fetch_all(&mut ***db)
        .await?;

        Ok(posts)
    }
}
