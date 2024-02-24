use rocket::{
    http::{Cookie, SameSite},
    serde::json::to_string,
};
use rocket_db_pools::Connection;
use serde_json::Error;
use sqlx::query_as;
use time::OffsetDateTime;

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
    pub fn new(token: User, expires_in: OffsetDateTime, creation_date: OffsetDateTime) -> Self {
        JWT {
            token,
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
    ) -> Self {
        UserToken {
            user_id,
            refresh_token,
            refresh_token_expires_in: expiry_date,
            refresh_token_creation_date: creation_date,
        }
    }

    pub async fn db_select_by_user_id(
        db: &mut Connection<DbConn>,
        user_id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        let user_token = query_as!(
            UserToken,
            r#"SELECT * FROM users_token WHERE user_id = $1"#,
            user_id
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(Some(user_token))
    }

    pub async fn db_delete_by_user_id(
        db: &mut Connection<DbConn>,
        user_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(r#"DELETE FROM users_token WHERE user_id = $1"#)
            .bind(user_id)
            .execute(&mut ***db)
            .await?;

        Ok(())
    }
}
