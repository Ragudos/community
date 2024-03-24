use rocket_db_pools::Connection;
use sqlx::{Postgres, Transaction};

use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityMembership;

impl CommunityMembership {
    pub async fn get_total(
        db: &mut Connection<DbConn>,
        id: &i64,
    ) -> Result<Option<i64>, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT COUNT(*)
                FROM community_memberships
                WHERE _community_id = $1
                "#,
            id
        )
        .fetch_one(&mut ***db)
        .await?
        .count)
    }

    pub async fn is_user_a_member(
        db: &mut Connection<DbConn>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM community_memberships
                    WHERE _community_id = $1
                    AND _user_id = $2
                ) AS "exists!"
                "#,
            community_id,
            user_id
        )
        .fetch_one(&mut ***db)
        .await?
        .exists)
    }

    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO community_memberships (_community_id, _user_id)
            VALUES ($1, $2);
            "#,
            community_id,
            user_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
