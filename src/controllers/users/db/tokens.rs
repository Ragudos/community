use rocket::{
    http::{Cookie, SameSite},
    serde::json::to_string,
};
use rocket_db_pools::Connection;
use serde_json::Error;
use sqlx::{query_as, Postgres, Transaction};
use time::{Duration, OffsetDateTime};

use crate::{
    helpers::db::DbConn,
    models::{
        users::metadata::{User, UserToken, JWT},
        JWT_NAME,
    },
};

use super::traits::Token;

impl Token for JWT {
    fn is_expired(&self) -> bool {
        self.expires_in < OffsetDateTime::now_utc()
    }
}

impl Token for UserToken {
    fn is_expired(&self) -> bool {
        self.refresh_token_expires_in < OffsetDateTime::now_utc()
    }
}

impl JWT {
    pub fn new(
        token: User,
        expires_in: OffsetDateTime,
        creation_date: OffsetDateTime,
        refresh_token: String,
    ) -> Self {
        JWT {
            token,
            refresh_token,
            expires_in,
            creation_date,
        }
    }

    pub fn to_cookie(&self) -> Result<Cookie<'static>, Error> {
        let stringified = to_string(self);

        match stringified {
            Ok(stringified) => Ok(Cookie::build((JWT_NAME, stringified))
                .same_site(SameSite::Strict)
                .path("/")
                .expires(self.expires_in)
                .secure(true)
                .http_only(true)
                .into()),
            Err(err) => Err(err),
        }
    }
}

impl UserToken {
    pub fn new(
        user_id: i32,
        refresh_token: String,
        expiry_date: OffsetDateTime,
        creation_date: OffsetDateTime,
    ) -> UserToken {
        UserToken {
            user_id,
            refresh_token,
            refresh_token_expires_in: expiry_date,
            refresh_token_creation_date: creation_date,
        }
    }

    pub async fn db_select_by_refresh_token(
        db: &mut Connection<DbConn>,
        refresh_token: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let user_token = query_as!(
            UserToken,
            r#"SELECT * FROM users_token WHERE refresh_token = $1"#,
            refresh_token
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(user_token)
    }

    pub async fn db_create(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i32,
        refresh_token: &str,
    ) -> Result<(), sqlx::Error> {
        let time_today = OffsetDateTime::now_utc();

        sqlx::query(
            r#"INSERT INTO users_token (user_id, refresh_token, refresh_token_expires_in, refresh_token_creation_date)
            VALUES ($1, $2, $3, $4)"#,
        )
        .bind(user_id)
        .bind(refresh_token)
        .bind(time_today.saturating_add(Duration::days(7)))
        .bind(time_today)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn db_update_refresh_token(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i32,
        refresh_token: &str,
    ) -> Result<(), sqlx::Error> {
        let time_today = OffsetDateTime::now_utc();

        sqlx::query(
            r#"UPDATE users_token SET refresh_token = $1, refresh_token_expires_in = $2, refresh_token_creation_date = $3 WHERE user_id = $4"#,
        )
        .bind(refresh_token)
        .bind(time_today.saturating_add(Duration::days(7)))
        .bind(time_today)
        .bind(user_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn db_select_by_user_id(
        db: &mut Connection<DbConn>,
        user_id: &i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        let user_token = query_as!(
            UserToken,
            r#"SELECT * FROM users_token WHERE user_id = $1"#,
            user_id
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(user_token)
    }

    pub async fn db_delete_by_refresh_token(
        db: &mut Connection<DbConn>,
        refresh_token: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(r#"DELETE FROM users_token WHERE refresh_token = $1"#)
            .bind(refresh_token)
            .execute(&mut ***db)
            .await?;

        Ok(())
    }
}
