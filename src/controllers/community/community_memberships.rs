use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::{helpers::db::DbConn, models::community::schema::CommunityMembership};

impl CommunityMembership {
    pub async fn get_total_by_uid(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<i64>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM community_memberships
            WHERE (
                SELECT _id from communities WHERE uid = $1
            ) = _community_id;
            "#,
            uid
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.count)
    }

    pub async fn is_user_a_member(
        db: &mut Connection<DbConn>,
        community_uid: &Uuid,
        user_uid: &Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM community_memberships
                WHERE (
                    SELECT _id from communities WHERE uid = $1
                ) = _community_id
                AND (
                    SELECT _id from users WHERE uid = $2
                ) = _user_id
            ) AS "exists!"
            "#,
            community_uid,
            user_uid
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.exists)
    }

    pub async fn create(
        db: &mut Connection<DbConn>,
        community_id: i64,
        user_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO community_memberships (_community_id, _user_id)
            VALUES ($1, $2);
            "#,
            community_id,
            user_id
        )
        .execute(&mut ***db)
        .await?;

        Ok(())
    }
}
