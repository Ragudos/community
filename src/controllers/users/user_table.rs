use rocket_db_pools::Connection;
use sqlx::{types::Uuid, Postgres, Transaction};

use crate::{helpers::db::DbConn, models::users::schema::UserTable};

impl UserTable {
    pub async fn is_name_taken(
        db: &mut Connection<DbConn>,
        display_name: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM users
                    WHERE display_name = $1
                ) AS exists
            "#,
            display_name
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.exists.map_or(false, |s| s))
    }

    /// To limit the number of communities a user can create
    pub async fn does_own_community(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM communities
                    WHERE owner_id = (
                        SELECT _id
                        FROM users
                        WHERE uid = $1
                    )
                ) AS exists
            "#,
            uid
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.exists.map_or(false, |s| s))
    }

    /// Returns the uid
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        display_name: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                INSERT INTO users (display_name)
                VALUES ($1)
                RETURNING uid
            "#,
            display_name,
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(result.uid)
    }
}
