use rocket_db_pools::Connection;
use sqlx::{types::Uuid, Postgres, Transaction};

use crate::{
    helpers::db::DbConn,
    models::users::schema::{FullName, UserCredentials},
};

impl UserCredentials {
    /// must be accessed after user is verified using 2FA or password.
    pub async fn get_email(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<String>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT email
                FROM user_credentials
                WHERE (
                    SELECT _id
                    FROM users
                    WHERE uid = $1
                ) = _id;
            "#,
            uid
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(result.map(|r| r.email))
    }

    pub async fn get_full_name(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<FullName>, sqlx::Error> {
        let result = sqlx::query_as!(
            FullName,
            r#"
                SELECT first_name, last_name
                FROM user_credentials
                WHERE (
                    SELECT _id
                    FROM users
                    WHERE uid = $1
                ) = _id;
            "#,
            uid
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(result)
    }

    pub async fn get_password_hash(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<String, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT password_hash
                FROM user_credentials
                WHERE (
                    SELECT _id
                    FROM users
                    WHERE uid = $1
                ) = _id;
            "#,
            uid
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.password_hash)
    }

    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        uid: &Uuid,
        email: Option<&str>,
        password_hash: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO user_credentials (_id, email, password_hash, first_name, last_name)
                VALUES (
                    (
                        SELECT _id
                        FROM users
                        WHERE uid = $1
                    ),
                    $2, $3, $4, $5
                )
            "#,
            uid,
            email,
            password_hash,
            first_name,
            last_name
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
