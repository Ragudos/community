use crate::{
    helpers::db::DbConn,
    models::community::schema::{Community, CommunityCategory},
};
use rocket_db_pools::Connection;
use sqlx::{types::BigDecimal, Error};

impl Community {
    pub async fn get_total_members_count_by_display_name(
        db: &mut Connection<DbConn>,
        display_name: &str,
    ) -> Result<Option<i64>, Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM community_memberships
            WHERE community_id = (
                SELECT id
                FROM communities
                WHERE display_name = $1
            )
            "#,
            display_name
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.count)
    }

    pub async fn get_total_members_count_by_id(
        db: &mut Connection<DbConn>,
        community_id: i32,
    ) -> Result<Option<i64>, Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM community_memberships
            WHERE community_id = $1
            "#,
            community_id
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.count)
    }

    pub async fn search_all_by_offset_weighted(
        db: &mut Connection<DbConn>,
        limit: &i64,
        offset: &i64,
        display_name: &str,
    ) -> Result<Vec<Community>, Error> {
        let communities = sqlx::query_as!(
            Community,
            r#"
            SELECT
            c.id,
            c.display_name,
            c.display_image,
            c.cover_image,
            c.description,
            c.owner_id,
            c.is_private,
            c.category as "category: CommunityCategory",
            created_at
            FROM communities c
            LEFT JOIN (
                SELECT c.id,
                SUM(members_count * 0.6) +
                SUM(posts_count * 0.15) +
                SUM(reactions_count * 0.25) AS weighted_score
                FROM communities c

                LEFT JOIN (
                    SELECT community_id, COUNT(DISTINCT user_id) AS members_count
                    FROM community_memberships
                    GROUP BY community_id
                ) m on c.id = m.community_id
                LEFT JOIN (
                    SELECT community_id, COUNT(*) AS posts_count
                    FROM community_posts
                    GROUP BY community_id
                ) p ON c.id = p.community_id
                LEFT JOIN (
                    SELECT cp.community_id, COUNT(*) AS reactions_count
                    FROM community_posts cp
                    LEFT JOIN reactions r ON cp.post_id = r.post_id
                    GROUP BY cp.community_id
                ) pr ON c.id = pr.community_id

                GROUP BY c.id
            ) cm ON c.id = cm.id
            WHERE similarity(c.display_name, $1) > 0.1
            ORDER BY COALESCE(cm.weighted_score, 0) DESC, c.created_at DESC
            LIMIT $2 OFFSET $3;
            "#,
            display_name,
            limit,
            offset
        )
        .fetch_all(&mut ***db)
        .await?;

        Ok(communities)
    }

    pub async fn get_all_by_offset_weighted(
        db: &mut Connection<DbConn>,
        limit: &i64,
        offset: &i64,
    ) -> Result<Vec<Community>, Error> {
        let communities = sqlx::query_as!(
            Community,
            r#"
            SELECT
            c.id,
            c.display_name,
            c.display_image,
            c.cover_image,
            c.description,
            c.owner_id,
            c.is_private,
            c.category as "category: CommunityCategory",
            created_at
            FROM communities c
            LEFT JOIN (
                SELECT c.id,
                SUM(members_count * 0.6) +
                SUM(posts_count * 0.15) +
                SUM(reactions_count * 0.25) AS weighted_score
                FROM communities c

                LEFT JOIN (
                    SELECT community_id, COUNT(DISTINCT user_id) AS members_count
                    FROM community_memberships
                    GROUP BY community_id
                ) m on c.id = m.community_id
                LEFT JOIN (
                    SELECT community_id, COUNT(*) AS posts_count
                    FROM community_posts
                    GROUP BY community_id
                ) p ON c.id = p.community_id
                LEFT JOIN (
                    SELECT cp.community_id, COUNT(*) AS reactions_count
                    FROM community_posts cp
                    LEFT JOIN reactions r ON cp.post_id = r.post_id
                    GROUP BY cp.community_id
                ) pr ON c.id = pr.community_id

                GROUP BY c.id
            ) cm ON c.id = cm.id
            ORDER BY COALESCE(cm.weighted_score, 0) DESC, c.created_at DESC
            LIMIT $1 OFFSET $2;
            "#,
            limit,
            offset
        )
        .fetch_all(&mut ***db)
        .await?;

        Ok(communities)
    }

    pub async fn get_communities_count(
        db: &mut Connection<DbConn>,
        display_name: Option<&str>,
    ) -> Result<Option<BigDecimal>, Error> {
        let result = match display_name {
            Some(name) => {
                if name.is_empty() {
                    sqlx::query!(
                        r#"
                            SELECT CEIL(COUNT(*)::NUMERIC / 20) AS count
                            FROM communities;
                        "#
                    )
                    .fetch_one(&mut ***db)
                    .await?
                    .count
                } else {
                    sqlx::query!(
                        r#"
                            SELECT CEIL(COUNT(*)::NUMERIC / 20) AS count
                            FROM communities
                            WHERE similarity(display_name, $1) > 0.1;
                        "#,
                        name
                    )
                    .fetch_one(&mut ***db)
                    .await?
                    .count
                }
            }
            None => {
                sqlx::query!(
                    r#"
                        SELECT CEIL(COUNT(*)::NUMERIC / 20) AS count
                        FROM communities;
                    "#
                )
                .fetch_one(&mut ***db)
                .await?
                .count
            }
        };

        Ok(result)
    }
}
