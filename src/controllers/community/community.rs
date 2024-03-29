use rocket_db_pools::Connection;
use sqlx::prelude::FromRow;
use sqlx::types::BigDecimal;
use sqlx::{Postgres, QueryBuilder, Transaction};

use crate::helpers::db::DbConn;
use crate::models::community::schema::{
    Community, CommunityAbout, CommunityHomepageCard, CommunityOfUser,
};
use crate::models::db::enums::CommunityCategory;

#[derive(FromRow)]
pub struct CountStruct {
    pub count: Option<BigDecimal>,
}

impl CommunityAbout {
    #[allow(non_snake_case)] // The macro builds what we return as, sqlx_query_as__id, _id being what is returned
    pub async fn get(
        db: &mut Connection<DbConn>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<Option<CommunityAbout>, sqlx::Error> {
        Ok(sqlx::query_as!(
            CommunityAbout,
            r#"
                SELECT
                c._id,
                display_name,
                display_image,
                description,
                cover_image,
                is_private,
                c.owner_id,
                owner_display_image,
                owner_display_name,
                total_members,
                total_online_members,
                total_admins,
                is_viewer_a_member
                FROM communities c
                LEFT JOIN (
                    SELECT
                    c2._id,
                    owner_display_image,
                    owner_display_name,
                    total_members AS total_members,
                    total_online_members,
                    total_admins,
                    is_viewer_a_member
                    FROM communities c2

                    LEFT JOIN (
                        SELECT _community_id, COALESCE(COUNT(*), 0) AS total_members, total_online_members
                        FROM community_memberships cm

                        LEFT JOIN (
                            SELECT _user_id, COALESCE(COUNT(*), 0) AS total_online_members
                            FROM online_sessions
                            WHERE _updated_at > NOW() - INTERVAL '10 minutes'

                            GROUP BY _user_id
                        ) os ON os._user_id = cm._user_id

                        GROUP BY _community_id, total_online_members
                    ) cm ON c2._id = cm._community_id

                    LEFT JOIN (
                        SELECT _community_id, COALESCE(COUNT(*), 0) AS total_admins
                        FROM community_memberships
                        WHERE role = 'admin'
                        
                        GROUP BY _community_id
                    ) ca ON c2._id = ca._community_id

                    LEFT JOIN (
                        SELECT _id,
                        display_name AS owner_display_name,
                        display_image AS owner_display_image
                        FROM users
                    ) u ON c2.owner_id = u._id

                    LEFT JOIN (
                        SELECT EXISTS (
                            SELECT 1
                            FROM community_memberships
                            WHERE _user_id = $2 AND _community_id = $1
                        ) AS is_viewer_a_member,
                        _community_id
                        FROM community_memberships

                        GROUP BY is_viewer_a_member, _community_id
                    ) io ON c2._id = io._community_id

                    GROUP BY
                    c2._id,
                    io.is_viewer_a_member,
                    owner_display_image,
                    owner_display_name,
                    total_members,
                    total_online_members,
                    total_admins
                ) cm ON c._id = cm._id

                WHERE c._id = $1
                "#,
            community_id,
            user_id
        )
        .fetch_optional(&mut ***db)
        .await?)
    }
}

impl Community {
    pub async fn get_owner_id(
        db: &mut Connection<DbConn>,
        community_id: &i64,
    ) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT owner_id
                FROM communities
                WHERE _id = $1
                "#,
            community_id
        )
        .fetch_one(&mut ***db)
        .await?
        .owner_id)
    }

    pub async fn change_join_process(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
    ) -> Result<bool, sqlx::Error> {
        let t = sqlx::query!(
            r#"
                UPDATE communities
                SET is_private = NOT is_private
                WHERE _id = $1
                RETURNING is_private
                "#,
            community_id
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(t.is_private)
    }

    pub async fn soft_delete(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                UPDATE communities
                SET is_deleted = true
                WHERE _id = $1
                "#,
            community_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Hard deletes a community from the database.
    pub async fn delete(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM communities
                WHERE _id = $1
                "#,
            community_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_name(
        db: &mut Connection<DbConn>,
        community_id: &i64,
    ) -> Result<Option<String>, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT display_name
                FROM communities
                WHERE _id = $1
                "#,
            community_id
        )
        .fetch_optional(&mut ***db)
        .await?
        .map(|t| t.display_name))
    }

    pub async fn update_name(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
        new_name: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                UPDATE communities
                SET display_name = $1
                WHERE _id = $2
            "#,
            new_name,
            community_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn is_user_owner(
        db: &mut Connection<DbConn>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<Option<bool>, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                    SELECT
                    owner_id
                    FROM communities
                    WHERE _id = $1
                "#,
            community_id
        )
        .fetch_optional(&mut ***db)
        .await?
        .map(|t| &t.owner_id == user_id))
    }

    pub async fn is_private(
        db: &mut Connection<DbConn>,
        community_id: &i64,
    ) -> Result<Option<bool>, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT is_private
                FROM communities
                WHERE _id = $1;
                "#,
            community_id
        )
        .fetch_optional(&mut ***db)
        .await?
        .map(|row| row.is_private))
    }

    pub async fn is_name_taken(
        db: &mut Connection<DbConn>,
        name: &str,
    ) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM communities
                    WHERE display_name = $1
                ) AS "exists!"
                "#,
            name
        )
        .fetch_one(&mut ***db)
        .await?
        .exists)
    }

    pub async fn get_communities_of_user(
        db: &mut Connection<DbConn>,
        user_id: &i64,
        offset: &i64,
        limit: &i64,
        get_owned_by_user: bool,
        get_joined_by_user: bool,
        display_name: Option<&str>,
        categories: Option<&[CommunityCategory]>,
    ) -> Result<Vec<CommunityOfUser>, sqlx::Error> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT
            c._id,
            display_name,
            categories AS "categories: _",
            display_image,
            cover_image,
            description,
            is_private,
            total_members,
            _joined_at,
            role AS "role: _"
            FROM communities c
            LEFT JOIN (
                SELECT _community_id,
                COALESCE(COUNT(*), 0) AS total_members,
                role AS "role: _"
                FROM community_memberships
                WHERE _user_id = 
            "#,
        );
        query.push_bind(user_id);
        query.push(
            r#"
            GROUP BY _community_id, role
            ) cm ON cm._community_id = c._id
            LEFT JOIN (
                SELECT a._created_at AS _joined_at, a._id
                    CASE
                        WHEN a.owner_id =
            "#,
        );
        query.push_bind(user_id);
        query.push(
            r#"
             THEN a._created_at
                        ELSE b._created_at
                    ENS AS _joined_at
                FROM communities a
                JOIN community_memberships b ON a._id = b._community_id
                WHERE b._user_id = 
            "#,
        );
        query.push_bind(user_id);
        query.push(
            r#"
                GROUP BY _joined_at, _id
            ) j ON j._id = c._id
            "#,
        );

        if get_joined_by_user && !get_owned_by_user {
            query.push(
                r#"
                WHERE c._id = (
                    SELECT _community_id
                    FROM community_memberships
                    WHERE _user_id = 
                "#,
            );
            query.push_bind(user_id);
            query.push(")");
        } else if !get_joined_by_user && get_owned_by_user {
            query.push(
                r#"
                WHERE c.owner_id = 
                "#,
            );
            query.push_bind(user_id);
        }

        if let Some(display_name) = display_name {
            query.push(
                r#"
                AND similarity(c.display_name,
                "#,
            );
            query.push_bind(display_name);
            query.push(") > 0.1");
        }

        if let Some(categories) = categories {
            query.push(
                r#"
                AND categories @> 
                "#,
            );
            query.push_bind(categories);
        }

        query.push(
            r#"
            ORDER BY _joined_at DESC
            LIMIT 
            "#,
        );
        query.push_bind(limit);
        query.push(" OFFSET ");
        query.push_bind(offset);

        Ok(query
            .build_query_as::<CommunityOfUser>()
            .fetch_all(&mut ***db)
            .await?)
    }

    pub async fn get_pagination(
        db: &mut Connection<DbConn>,
        limit: &i64,
        categories: Option<&[CommunityCategory]>,
        display_name: Option<&str>,
    ) -> Result<Option<BigDecimal>, sqlx::Error> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT CEIL(COUNT(*)::NUMERIC / ?) AS count
            FROM communityies
            WHERE (
                (display_name IS NOT NULL AND display_image != '')
                OR (cover_Image IS NOT NULL AND cover_image != '')
            )
            "#,
        );
        query.push_bind(limit);

        if let Some(categories) = categories {
            query.push(
                r#"
                AND categories @> 
                "#,
            );
            query.push_bind(categories);
        }

        if let Some(display_name) = display_name {
            query.push(
                r#"
                AND similarity(display_name,
                "#,
            );
            query.push_bind(display_name);
            query.push(") > 0.1");
        }

        Ok(query
            .build_query_as::<CountStruct>()
            .fetch_one(&mut ***db)
            .await?
            .count)
    }

    pub async fn get_by_weighted_score(
        db: &mut Connection<DbConn>,
        offset: &i64,
        limit: &i64,
        categories: Option<&[CommunityCategory]>,
        display_name: Option<&str>,
    ) -> Result<Vec<CommunityHomepageCard>, sqlx::Error> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT
            c._id,
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
                COALESCE(total_members, 0) AS total_members,
                SUM(total_members * 0.4) +
                SUM(posts_count * 0.15) +
                SUM(post_reactions_count * 0.2) +
                SUM(comments_count * 0.25) AS weighted_score
                FROM communities c

                LEFT JOIN (
                    SELECT _community_id, COALESCE(COUNT(*), 0) AS total_members
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

                GROUP BY c._id, total_members
            ) cm ON c._id = cm._id
            WHERE (
                (display_image IS NOT NULL AND cover_image IS NOT NULL)
                OR (display_image != '' AND cover_image != '')
            )
            "#,
        );

        if let Some(categories) = categories {
            query.push(
                r#"
                AND categories @> 
                "#,
            );
            query.push_bind(categories);
        }

        if let Some(display_name) = display_name {
            query.push(
                r#"
                AND similarity(display_name,
                "#,
            );
            query.push_bind(display_name);
            query.push(") > 0.1");
        }

        query.push(
            r#"
            ORDER BY COALESCE(cm.weighted_score, 0) DESC, c._created_at DESC
            LIMIT
            "#,
        );
        query.push_bind(limit);
        query.push(" OFFSET ");
        query.push_bind(offset * limit);

        Ok(query
            .build_query_as::<CommunityHomepageCard>()
            .fetch_all(&mut ***db)
            .await?)
    }

    #[allow(non_snake_case)] // The macro builds what we return as, sqlx_query_as__id, _id being what is returned
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        display_name: &str,
        description: &str,
        owner_id: &i64,
    ) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                INSERT INTO communities (display_name, description, owner_id)
                VALUES ($1, $2, $3)
                RETURNING _id;
                "#,
            display_name,
            description,
            owner_id
        )
        .fetch_one(&mut **tx)
        .await?
        ._id)
    }
}
