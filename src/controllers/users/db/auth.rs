use rocket_db_pools::Connection;
use sqlx::{postgres::PgQueryResult, query, query_as, Error, Postgres, Transaction};

use crate::{
    helpers::db::DbConn,
    models::users::metadata::{Gender, User, UserCredentials, UserMetadata},
};

impl User {
    pub async fn is_name_taken(db: &mut Connection<DbConn>, name: &str) -> Result<bool, Error> {
        let result = query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM users
                    WHERE display_name = $1
                );
            "#,
            name
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    /// This is in a transaction to make sure that
    /// the user's metadata and credentials get inserted.
    /// If either or both fails, then the operation
    /// will fail and nothing will get inserted.
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        display_name: &str,
    ) -> Result<User, Error> {
        query_as!(
            User,
            r#"
                INSERT INTO users
                (display_name)
                VALUES ($1)
                RETURNING *;
            "#,
            display_name,
        )
        .fetch_one(&mut **tx)
        .await
    }
}

impl UserMetadata {
    /// This is in a transaction to make sure that
    /// the user's metadata and credentials get inserted.
    /// If either or both fails, then the operation
    /// will fail and nothing will get inserted.
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i32,
        gender: &Gender,
        is_private: bool,
    ) -> Result<PgQueryResult, Error> {
        query!(
            r#"
                INSERT INTO users_metadata
                (id, gender, is_private)
                VALUES ($1, $2, $3);
            "#,
            user_id,
            gender as &Gender,
            is_private
        )
        .execute(&mut **tx)
        .await
    }
}

impl UserCredentials {
    /// This is in a transaction to make sure that
    /// the user's metadata and credentials get inserted.
    /// If either or both fails, then the operation
    /// will fail and nothing will get inserted.
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i32,
        password_hash: &str,
    ) -> Result<PgQueryResult, Error> {
        query!(
            r#"
                INSERT INTO users_credentials
                (id, password_hash)
                VALUES ($1, $2)
            "#,
            user_id,
            password_hash,
        )
        .execute(&mut **tx)
        .await
    }
}
