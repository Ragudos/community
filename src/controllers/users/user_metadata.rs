use rocket_db_pools::Connection;
use sqlx::{types::Uuid, Postgres, Transaction};

use crate::{
    helpers::db::DbConn,
    models::{
        db::enums::{AccountStatus, Gender, Occupation},
        users::schema::UserMetadata,
    },
};

impl UserMetadata {
    pub async fn get_all(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<UserMetadata>, sqlx::Error> {
        let result = sqlx::query_as!(
            UserMetadata,
            r#"
                SELECT
                occupation as "occupation: Occupation",
                gender as "gender: Gender",
                biography,
                is_private,
                account_status as "account_status: AccountStatus"
                FROM user_metadata
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

    pub async fn get_occupation(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<Occupation>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT occupation as "occupation: Occupation"
                FROM user_metadata
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

        Ok(result.occupation)
    }

    pub async fn get_gender(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Gender, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                Select gender as "gender: Gender"
                FROM user_metadata
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

        Ok(result.gender)
    }

    pub async fn get_biography(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<String>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT biography
                FROM user_metadata
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

        Ok(result.biography)
    }

    pub async fn get_is_private(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT is_private
                FROM user_metadata
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

        Ok(result.is_private)
    }

    pub async fn get_account_status(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<AccountStatus, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT account_status as "account_status: AccountStatus"
                FROM user_metadata
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

        Ok(result.account_status)
    }

    pub async fn create(tx: &mut Transaction<'_, Postgres>, uid: &Uuid) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            r#"
                INSERT INTO user_metadata
                (_id)
                VALUES (
                    (SELECT _id FROM users WHERE uid = $1)
                );
            "#,
            uid,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
