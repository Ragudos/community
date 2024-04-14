use rocket_db_pools::Connection;
use sqlx::{Postgres, Transaction};

use crate::helpers::db::DbConn;
use crate::models::users::schema::UserTable;

impl UserTable {
    pub async fn get_display_image(
        db: &mut Connection<DbConn>,
        user_id: i64,
    ) -> Result<Option<String>, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT display_image
                FROM users
                WHERE _id = $1
                "#,
            user_id
        )
        .fetch_one(&mut ***db)
        .await?
        .display_image)
    }

    pub async fn is_name_taken(
        db: &mut Connection<DbConn>,
        display_name: &str,
    ) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query!(
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
        .await?
        .exists
        .unwrap_or(false))
    }

    pub async fn count_of_owned_communities(
        db: &mut Connection<DbConn>,
        user_id: &i64,
    ) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT COUNT(*)
                FROM communities
                WHERE owner_id = $1
                "#,
            user_id
        )
        .fetch_one(&mut ***db)
        .await?
        .count
        .unwrap_or(0))
    }

    #[allow(non_snake_case)]
    /// Returns the uid
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        display_name: &str,
    ) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                    INSERT INTO users (display_name)
                    VALUES ($1)
                    RETURNING _id
                "#,
            display_name,
        )
        .fetch_one(&mut **tx)
        .await?
        ._id)
    }
}
