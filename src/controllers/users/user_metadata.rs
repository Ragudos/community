use rocket_db_pools::Connection;
use sqlx::{Postgres, Transaction};

use crate::helpers::db::DbConn;
use crate::models::db::enums::{AccountStatus, Gender, Occupation};
use crate::models::users::schema::UserMetadata;

impl UserMetadata {
    pub async fn get(
        db: &mut Connection<DbConn>,
        id: &i64,
    ) -> Result<Option<UserMetadata>, sqlx::Error> {
        Ok(sqlx::query_as!(
            UserMetadata,
            r#"
                    SELECT
                    occupation as "occupation: Occupation",
                    gender as "gender: Gender",
                    biography,
                    is_private,
                    account_status as "account_status: AccountStatus"
                    FROM user_metadata
                    WHERE _id = $1;
                "#,
            id
        )
        .fetch_optional(&mut ***db)
        .await?)
    }

    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        id: &i64,
    ) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            r#"
                INSERT INTO user_metadata (_id)
                VALUES ($1);
            "#,
            id,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
