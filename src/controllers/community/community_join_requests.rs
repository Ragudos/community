use rocket_db_pools::Connection;
use sqlx::{Postgres, Transaction};

use crate::helpers::db::DbConn;
use crate::models::community::schema::CommunityJoinRequest;
use crate::models::db::enums::RequestStatus;

impl CommunityJoinRequest {
    /// Used for when a community owner rejects all join request OR
    /// it becomes public.
    pub async fn delete_all_join_requests_of_community(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM community_join_requests
            WHERE _community_id = $1;
            "#,
            community_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// To delete previous non-pending requests of a user
    /// in a community. We don't delete them right away when rejected/accepted to show
    /// the user the status of their join requests.
    pub async fn delete_non_pending_join_requests_of_user(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM community_join_requests
            WHERE (_community_id = $1
            AND _user_id = $2)
            AND status != $3;
            "#,
            community_id,
            user_id,
            RequestStatus::Pending as RequestStatus
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// There can only be one pending join request for a private community.
    pub async fn delete_pending_join_request_of_user(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM community_join_requests
            WHERE (_community_id = $1
            AND _user_id = $2)
            AND status = $3;
            "#,
            community_id,
            user_id,
            RequestStatus::Pending as RequestStatus
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn delete_join_request(
        tx: &mut Transaction<'_, Postgres>,
        id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM community_join_requests
            WHERE _id = $1;
            "#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn delete_pending_join_request(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM community_join_requests
            WHERE (_community_id = $1
            AND _user_id = $2)
            AND status = $3;
            "#,
            community_id,
            user_id,
            RequestStatus::Pending as RequestStatus
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Deletes ALL join requests of a user, the pending, accepted, and rejected ones.
    /// If you wish to remove only the pending requests, use `delete_pending_join_requests_of_user`.
    pub async fn delete_join_requests_of_user(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM community_join_requests
            WHERE _community_id = $1
            AND _user_id = $2;
            "#,
            community_id,
            user_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn did_user_request_to_join(
        db: &mut Connection<DbConn>,
        community_id: &i64,
        user_id: &i64,
    ) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM community_join_requests
                    WHERE (_community_id = $1
                    AND _user_id = $2)
                    AND status = $3
                ) AS "exists!"
                "#,
            community_id,
            user_id,
            RequestStatus::Pending as RequestStatus
        )
        .fetch_one(&mut ***db)
        .await?
        .exists)
    }

    #[allow(non_snake_case)]
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        community_id: &i64,
        user_id: &i64,
        reason: &str,
    ) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
            INSERT INTO community_join_requests (_community_id, _user_id, reason)
            VALUES ($1, $2, $3)
            RETURNING _id;
            "#,
            community_id,
            user_id,
            reason
        )
        .fetch_one(&mut **tx)
        .await?
        ._id)
    }
}
