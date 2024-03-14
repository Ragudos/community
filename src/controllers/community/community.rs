use rocket_db_pools::Connection;
use sqlx::{
    types::{BigDecimal, Uuid},
    Postgres, Transaction,
};

use crate::{
    helpers::db::DbConn,
    models::{
        community::schema::{
            Community, CommunityHomepageCard, CommunityOfUser, CommunityWithTotalMembers,
        },
        db::enums::CommunityCategory,
    },
};

impl Community {
    pub async fn is_name_taken(
        db: &mut Connection<DbConn>,
        name: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM communities
                WHERE display_name = $1
            ) as "exists!"
            "#,
            name
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.exists)
    }

    pub async fn get_communities_of_user_by_uid(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
        limit: &i64,
        offset: &i64,
        categories: &Vec<CommunityCategory>,
        display_name: &str,
    ) -> Result<Vec<CommunityOfUser>, sqlx::Error> {
        let result = sqlx::query_as!(
            CommunityOfUser,
            r#"
            SELECT
            uid,
            display_name,
            categories AS "categories: _",
            display_image,
            cover_image,
            description,
            is_private,
            total_members,
            joined_at,
            role AS "role: _"
            FROM communities c
            LEFT JOIN (
                SELECT _community_id, COALESCE(COUNT(*), 0) AS total_members, _created_at AS joined_at, role
                FROM community_memberships
                WHERE _user_id = (SELECT _id FROM users WHERE uid = $1)

                GROUP BY _community_id, _created_at, role
            ) cm ON cm._community_id = c._id
            WHERE categories @> $2 AND similarity(display_name, $3) > 0.1
            ORDER BY joined_at DESC
            LIMIT $4 OFFSET $5
            "#,
            uid,
            categories as &Vec<CommunityCategory>,
            display_name,
            limit,
            offset
        ).fetch_all(&mut ***db).await?;

        Ok(result)
    }

    pub async fn get_pagination_filtered_by_category_and_display_name(
        db: &mut Connection<DbConn>,
        limit: i64,
        categories: &Vec<CommunityCategory>,
        display_name: &str,
    ) -> Result<Option<BigDecimal>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT CEIL(COUNT(*)::NUMERIC / $1) AS count
            FROM communities
            WHERE categories @> $2
            AND similarity(display_name, $3) > 0.1
            AND (display_image IS NOT NULL AND cover_image IS NOT NULL
            OR (display_image != '' AND cover_image != '' ));
            "#,
            BigDecimal::from(limit),
            categories as &Vec<CommunityCategory>,
            display_name
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.count)
    }

    pub async fn get_pagination_filtered_by_category(
        db: &mut Connection<DbConn>,
        limit: i64,
        categories: &Vec<CommunityCategory>,
    ) -> Result<Option<BigDecimal>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT CEIL(COUNT(*)::NUMERIC / $1) AS count
            FROM communities
            WHERE categories @> $2
            AND (display_image IS NOT NULL AND cover_image IS NOT NULL
            OR (display_image != '' AND cover_image != ''));
            "#,
            BigDecimal::from(limit),
            categories as &Vec<CommunityCategory>
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.count)
    }

    pub async fn get_pagination_count_filtered_by_display_name(
        db: &mut Connection<DbConn>,
        limit: i64,
        display_name: &str,
    ) -> Result<Option<BigDecimal>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT CEIL(COUNT(*)::NUMERIC / $1) AS count
                FROM communities
                WHERE similarity(display_name, $2) > 0.1
                AND (display_image IS NOT NULL AND cover_image IS NOT NULL
                OR (display_image != '' AND cover_image != ''));
                "#,
            BigDecimal::from(limit),
            display_name
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.count)
    }

    pub async fn get_pagination_count(
        db: &mut Connection<DbConn>,
        limit: i64,
    ) -> Result<Option<BigDecimal>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT CEIL(COUNT(*)::NUMERIC / $1) AS count
                FROM communities
                WHERE display_image IS NOT NULL AND cover_image IS NOT NULL
                OR (display_image != '' AND cover_image != '');
                "#,
            BigDecimal::from(limit)
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.count)
    }

    pub async fn get_by_uid(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<CommunityWithTotalMembers>, sqlx::Error> {
        let result = sqlx::query_as!(
            CommunityWithTotalMembers,
            r#"
            SELECT
            uid,
            display_name,
            categories as "categories: _",
            description,
            owner_id,
            is_private,
            display_image,
            cover_image,
            total_members
            FROM communities c
            LEFT JOIN (
                SELECT _community_id, COALESCE(COUNT(*), 0) AS total_members
                FROM community_memberships
                WHERE _community_id = (
                    SELECT _id FROM communities WHERE uid = $1
                )
                GROUP BY _community_id
            ) cm ON cm._community_id = c._id
            WHERE uid = $1;
            "#,
            uid
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(result)
    }

    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        display_name: &str,
        description: &str,
        owner_uid: &Uuid,
    ) -> Result<String, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO communities (display_name, description, owner_id)
            VALUES ($1, $2, (SELECT _id FROM users WHERE uid = $3))
            RETURNING uid;
            "#,
            display_name,
            description,
            owner_uid
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(result.uid.to_string())
    }

    pub async fn search_all_by_category_and_display_name_and_offset_and_weighted_score(
        db: &mut Connection<DbConn>,
        offset: &i64,
        limit: &i64,
        categories: &Vec<CommunityCategory>,
        display_name: &str,
    ) -> Result<Vec<CommunityHomepageCard>, sqlx::Error> {
        let communities = sqlx::query_as!(
            CommunityHomepageCard,
            r#"
            SELECT
            uid,
            display_name,
            display_image,
            cover_image,
            description,
            is_private,
            total_members
            FROM communities c
            LEFT JOIN (
                SELECT
                c._id,
                members_count AS total_members,
                SUM(members_count * 0.4) +
                SUM(posts_count * 0.15) +
                SUM(post_reactions_count * 0.1) +
                SUM(comments_reactions_count * 0.1) +
                SUM(comments_count * 0.25) AS weighted_score
                FROM communities c

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS members_count
                    FROM community_memberships
                    GROUP BY _community_id
                ) m ON c._id = m._community_id

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS posts_count
                    FROM community_posts
                    GROUP BY _community_id
                ) cp ON c._id = cp._community_id

                LEFT JOIN (
                    SELECT cp._community_id, post_reactions_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT pr._post_id, COALESCE(COUNT(*), 0) AS post_reactions_count
                        FROM post_reactions pr
                        GROUP BY pr._post_id
                    ) pr ON cp._post_id = pr._post_id
                    GROUP BY cp._community_id, post_reactions_count
                ) pr ON c._id = pr._community_id

                LEFT JOIN (
                    SELECT cp._community_id, comments_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT co._post_id, COALESCE(COUNT(*), 0) AS comments_count
                        FROM comments co
                        GROUP BY co._post_id
                    ) co ON cp._post_id = co._post_id
                    GROUP BY cp._community_id, comments_count
                ) co ON c._id = co._community_id

                LEFT JOIN (
                    SELECT cp._community_id, comments_reactions_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT co._id, co._post_id, comments_reactions_count
                        FROM comments co
                        LEFT JOIN (
                            SELECT cr._comment_id, COALESCE(COUNT(*), 0) AS comments_reactions_count
                            FROM comment_reactions cr
                            GROUP BY cr._comment_id
                        ) cr ON co._id = cr._comment_id

                        GROUP BY co._post_id, co._id, comments_reactions_count
                    ) co ON cp._post_id = co._post_id

                    GROUP BY cp._community_id, comments_reactions_count
                ) cr ON c._id = cr._community_id

               GROUP BY c._id, m.members_count
            ) cm ON c._id = cm._id
            WHERE
            categories @> $1 AND similarity(c.display_name, $2) > 0.1
            AND (display_image IS NOT NULL AND cover_image IS NOT NULL
            OR (display_image != '' AND cover_image != ''))
            ORDER BY COALESCE(cm.weighted_score, 0) DESC, c._created_at DESC
            LIMIT $3 OFFSET $4;
            "#,
            categories as &Vec<CommunityCategory>,
            display_name,
            limit,
            offset * limit,
        )
        .fetch_all(&mut ***db)
        .await?;

        Ok(communities)
    }

    pub async fn search_all_by_category_and_offset_and_weighted_score(
        db: &mut Connection<DbConn>,
        offset: &i64,
        limit: &i64,
        categories: &Vec<CommunityCategory>,
    ) -> Result<Vec<CommunityHomepageCard>, sqlx::Error> {
        let communities = sqlx::query_as!(
            CommunityHomepageCard,
            r#"
            SELECT
            uid,
            display_name,
            display_image,
            cover_image,
            description,
            is_private,
            total_members
            FROM communities c
            LEFT JOIN (
                SELECT
                c._id,
                COALESCE(members_count, 0) AS total_members,
                SUM(members_count * 0.4) +
                SUM(posts_count * 0.15) +
                SUM(post_reactions_count * 0.1) +
                SUM(comments_reactions_count * 0.1) +
                SUM(comments_count * 0.25) AS weighted_score
                FROM communities c

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS members_count
                    FROM community_memberships
                    GROUP BY _community_id
                ) m ON c._id = m._community_id

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS posts_count
                    FROM community_posts
                    GROUP BY _community_id
                ) cp ON c._id = cp._community_id

                LEFT JOIN (
                    SELECT cp._community_id, post_reactions_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT pr._post_id, COALESCE(COUNT(*), 0) AS post_reactions_count
                        FROM post_reactions pr
                        GROUP BY pr._post_id
                    ) pr ON cp._post_id = pr._post_id
                    GROUP BY cp._community_id, post_reactions_count
                ) pr ON c._id = pr._community_id

                LEFT JOIN (
                    SELECT cp._community_id, comments_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT co._post_id, COALESCE(COUNT(*), 0) AS comments_count
                        FROM comments co
                        GROUP BY co._post_id
                    ) co ON cp._post_id = co._post_id
                    GROUP BY cp._community_id, comments_count
                ) co ON c._id = co._community_id

                LEFT JOIN (
                    SELECT cp._community_id, comments_reactions_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT co._id, co._post_id, comments_reactions_count
                        FROM comments co
                        LEFT JOIN (
                            SELECT cr._comment_id, COALESCE(COUNT(*), 0) AS comments_reactions_count
                            FROM comment_reactions cr
                            GROUP BY cr._comment_id
                        ) cr ON co._id = cr._comment_id

                        GROUP BY co._post_id, co._id, comments_reactions_count
                    ) co ON cp._post_id = co._post_id

                    GROUP BY cp._community_id, comments_reactions_count
                ) cr ON c._id = cr._community_id

               GROUP BY c._id, m.members_count
            ) cm ON c._id = cm._id
            WHERE
            categories @> $1
            AND (display_image IS NOT NULL AND cover_image IS NOT NULL
            OR (display_image != '' AND cover_image != ''))
            ORDER BY COALESCE(cm.weighted_score, 0) DESC, c._created_at DESC
            LIMIT $2 OFFSET $3;
            "#,
            categories as &Vec<CommunityCategory>,
            limit,
            offset * limit,
        )
        .fetch_all(&mut ***db)
        .await?;

        Ok(communities)
    }

    pub async fn search_all_by_display_name_and_offset_and_weighted_score(
        db: &mut Connection<DbConn>,
        offset: &i64,
        limit: &i64,
        display_name: &str,
    ) -> Result<Vec<CommunityHomepageCard>, sqlx::Error> {
        let communities = sqlx::query_as!(
            CommunityHomepageCard,
            r#"
            SELECT
            uid,
            display_name,
            display_image,
            cover_image,
            description,
            is_private,
            total_members
            FROM communities c
            LEFT JOIN (
                SELECT
                c._id,
                COALESCE(members_count, 0) AS total_members,
                SUM(members_count * 0.4) +
                SUM(posts_count * 0.15) +
                SUM(post_reactions_count * 0.1) +
                SUM(comments_reactions_count * 0.1) +
                SUM(comments_count * 0.25) AS weighted_score
                FROM communities c

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS members_count
                    FROM community_memberships
                    GROUP BY _community_id
                ) m ON c._id = m._community_id

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS posts_count
                    FROM community_posts
                    GROUP BY _community_id
                ) cp ON c._id = cp._community_id

                LEFT JOIN (
                    SELECT cp._community_id, post_reactions_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT pr._post_id, COALESCE(COUNT(*), 0) AS post_reactions_count
                        FROM post_reactions pr
                        GROUP BY pr._post_id
                    ) pr ON cp._post_id = pr._post_id
                    GROUP BY cp._community_id, post_reactions_count
                ) pr ON c._id = pr._community_id

                LEFT JOIN (
                    SELECT cp._community_id, comments_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT co._post_id, COALESCE(COUNT(*), 0) AS comments_count
                        FROM comments co
                        GROUP BY co._post_id
                    ) co ON cp._post_id = co._post_id
                    GROUP BY cp._community_id, comments_count
                ) co ON c._id = co._community_id

                LEFT JOIN (
                    SELECT cp._community_id, comments_reactions_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT co._id, co._post_id, comments_reactions_count
                        FROM comments co
                        LEFT JOIN (
                            SELECT cr._comment_id, COAlESCE(COUNT(*), 0) AS comments_reactions_count
                            FROM comment_reactions cr
                            GROUP BY cr._comment_id
                        ) cr ON co._id = cr._comment_id

                        GROUP BY co._post_id, co._id, comments_reactions_count
                    ) co ON cp._post_id = co._post_id

                    GROUP BY cp._community_id, comments_reactions_count
                ) cr ON c._id = cr._community_id

               GROUP BY c._id, m.members_count
            ) cm ON c._id = cm._id
            WHERE
            similarity(c.display_name, $1) > 0.1
            AND (display_image IS NOT NULL AND cover_image IS NOT NULL
            OR (display_image != '' AND cover_image != ''))
            ORDER BY COALESCE(cm.weighted_score, 0) DESC, c._created_at DESC
            LIMIT $2 OFFSET $3;
            "#,
            display_name,
            limit,
            offset * limit,
        )
        .fetch_all(&mut ***db)
        .await?;

        Ok(communities)
    }

    pub async fn get_all_by_offset_and_weighted_score(
        db: &mut Connection<DbConn>,
        offset: &i64,
        limit: &i64,
    ) -> Result<Vec<CommunityHomepageCard>, sqlx::Error> {
        let communities = sqlx::query_as!(
            CommunityHomepageCard,
            r#"
            SELECT
            uid,
            display_name,
            display_image,
            cover_image,
            description,
            is_private,
            total_members
            FROM communities c
            LEFT JOIN (
                SELECT
                c._id,
                COALESCE(members_count, 0) AS total_members,
                SUM(members_count * 0.4) +
                SUM(posts_count * 0.15) +
                SUM(post_reactions_count * 0.1) +
                SUM(comments_reactions_count * 0.1) +
                SUM(comments_count * 0.25) AS weighted_score
                FROM communities c

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS members_count
                    FROM community_memberships
                    GROUP BY _community_id
                ) m ON c._id = m._community_id

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS posts_count
                    FROM community_posts
                    GROUP BY _community_id
                ) cp ON c._id = cp._community_id

                LEFT JOIN (
                    SELECT cp._community_id, post_reactions_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT pr._post_id, COALESCE(COUNT(*), 0) AS post_reactions_count
                        FROM post_reactions pr
                        GROUP BY pr._post_id
                    ) pr ON cp._post_id = pr._post_id
                    GROUP BY cp._community_id, post_reactions_count
                ) pr ON c._id = pr._community_id

                LEFT JOIN (
                    SELECT cp._community_id, comments_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT co._post_id, COALESCE(COUNT(*), 0) AS comments_count
                        FROM comments co
                        GROUP BY co._post_id
                    ) co ON cp._post_id = co._post_id
                    GROUP BY cp._community_id, comments_count
                ) co ON c._id = co._community_id

                LEFT JOIN (
                    SELECT cp._community_id, comments_reactions_count
                    FROM community_posts cp
                    LEFT JOIN (
                        SELECT co._id, co._post_id, comments_reactions_count
                        FROM comments co
                        LEFT JOIN (
                            SELECT cr._comment_id, COALESCE(COUNT(*), 0) AS comments_reactions_count
                            FROM comment_reactions cr
                            GROUP BY cr._comment_id
                        ) cr ON co._id = cr._comment_id

                        GROUP BY co._post_id, co._id, comments_reactions_count
                    ) co ON cp._post_id = co._post_id

                    GROUP BY cp._community_id, comments_reactions_count
                ) cr ON c._id = cr._community_id

               GROUP BY c._id, m.members_count
            ) cm ON c._id = cm._id
            WHERE display_image IS NOT NULL AND cover_image IS NOT NULL
            OR (display_image != '' AND cover_image != '')
            ORDER BY COALESCE(cm.weighted_score, 0) DESC, c._created_at DESC
            LIMIT $1 OFFSET $2;
            "#,
            limit,
            offset * limit
        )
        .fetch_all(&mut ***db)
        .await?;

        Ok(communities)
    }
}
