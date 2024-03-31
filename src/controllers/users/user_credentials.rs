use rocket_db_pools::Connection;
use sqlx::{Postgres, Transaction};

use crate::helpers::db::DbConn;
use crate::models::users::schema::{
    EmailStruct, FullName, PasswordStruct, UserCredentials,
};

impl UserCredentials {
    /// must be accessed after user is verified using 2FA or password.
    pub async fn get_email(
        db: &mut Connection<DbConn>,
        id: &i64,
    ) -> Result<Option<EmailStruct>, sqlx::Error> {
        Ok(sqlx::query_as!(
            EmailStruct,
            r#"
                    SELECT email
                    FROM user_credentials
                    WHERE _id = $1;
                "#,
            id
        )
        .fetch_optional(&mut ***db)
        .await?)
    }

    pub async fn get_full_name(
        db: &mut Connection<DbConn>,
        id: &i64,
    ) -> Result<Option<FullName>, sqlx::Error> {
        Ok(sqlx::query_as!(
            FullName,
            r#"
                    SELECT first_name, last_name
                    FROM user_credentials
                    WHERE _id = $1
                "#,
            id
        )
        .fetch_optional(&mut ***db)
        .await?)
    }

    pub async fn get_password_hash(
        db: &mut Connection<DbConn>,
        id: &i64,
    ) -> Result<Option<PasswordStruct>, sqlx::Error> {
        Ok(sqlx::query_as!(
            PasswordStruct,
            r#"
                    SELECT password_hash
                    FROM user_credentials
                    WHERE _id = $1
                "#,
            id
        )
        .fetch_optional(&mut ***db)
        .await?)
    }

    /// To verify if a user with that name exists whilst getting their password for verification.
    /// This is to optimize our logic for the /login endpoint
    pub async fn get_password_hash_by_name(
        db: &mut Connection<DbConn>,
        display_name: &str,
    ) -> Result<Option<PasswordStruct>, sqlx::Error> {
        Ok(sqlx::query_as!(
            PasswordStruct,
            r#"
                    SELECT password_hash
                    FROM user_credentials
                    WHERE (
                        SELECT _id
                        FROM users
                        WHERE display_name = $1
                    ) = _id;
                "#,
            display_name
        )
        .fetch_optional(&mut ***db)
        .await?)
    }

    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        id: &i64,
        email: Option<&str>,
        password_hash: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO user_credentials (_id, email, password_hash, first_name, last_name)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            id,
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
