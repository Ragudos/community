use rocket_db_pools::Connection;

use crate::{
    helpers::db::DbConn,
    models::{
        community::schema::{Community, CommunityCategory, CommunityMembership},
        users::metadata::UserRole,
    },
};

use sqlx::{Error, Postgres, Transaction};

impl Community {
    pub async fn is_name_taken(db: &mut Connection<DbConn>, name: &str) -> Result<bool, Error> {
        let result = sqlx::query!(
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
        .await?;

        Ok(result.exists)
    }

    pub async fn store(
        tx: &mut Transaction<'_, Postgres>,
        display_name: &str,
        display_image: &str,
        cover_image: &str,
        description: &str,
        is_private: bool,
        category: Option<&CommunityCategory>,
        owner_id: &i32,
    ) -> Result<i32, Error> {
        let res = sqlx::query!(
            r#"
            INSERT INTO communities (display_name, display_image, cover_image, description, is_private, category, owner_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id;
            "#,
            display_name,
            display_image,
            cover_image,
            description,
            is_private,
            category as Option<&CommunityCategory>,
            owner_id
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(res.id)
    }
}

impl CommunityMembership {
    pub async fn is_member_in_community(
        db: &mut Connection<DbConn>,
        user_id: &i32,
        community_id: &i32,
    ) -> Result<bool, Error> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM community_memberships
                WHERE user_id = $1 AND community_id = $2
            ) AS "exists!"
            "#,
            user_id,
            community_id
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.exists)
    }

    pub async fn store(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i32,
        community_id: &i32,
        role: UserRole,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO community_memberships (user_id, community_id, role)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            community_id,
            role as UserRole
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
