use rocket_db_pools::Connection;
use sqlx::QueryBuilder;

use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityPost;

impl CommunityPost {
    pub async fn get_community_posts(
        db: &mut Connection<DbConn>,
        community_id: &i64,
        search: Option<&str>,
        offset: &i64,
        limit: &i64,
    ) -> Result<Vec<CommunityPost>, sqlx::Error> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT
            _id,
            _created_at,
            is_pinned,
            caption,
            content,
            links,
            images,
            videos,
            owner_id,
            FROM posts
            WHERE _id = (
                SELECT _post_id
                FROM community_posts
                WHERE _community_id = ?
            )
            "#,
        );
        query.push_bind(community_id);

        if let Some(search) = search {
            query.push(
                r#"
                AND (
                    similarity(caption, ?) > 0.1
                    OR similarity(
                        (SELECT display_name FROM users WHERE _id = owner_id),
                        ?
                    ) > 0.1
                )
                "#,
            );
            query.push_bind(search);
        }

        query.push(
            r#"
            ORDER BY _created_at DESC
            OFFSET ?
            "#,
        );
        query.push_bind(offset);
        query.push(
            r#"
            LIMIT ?
            "#,
        );
        query.push_bind(limit);

        let cp: Vec<CommunityPost> = query.build_query_as().fetch_all(&mut ***db).await?;

        Ok(cp)
    }
}
