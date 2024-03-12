use std::str::FromStr;

use rocket::http::{Cookie, SameSite};
use rocket_db_pools::Connection;
use sqlx::types::Uuid;
use time::{Duration, OffsetDateTime};

use crate::{
    helpers::db::DbConn,
    models::{users::schema::UserJWT, JWT_NAME},
};

impl UserJWT {
    pub fn to_cookie(&self) -> Result<Cookie<'static>, serde_json::Error> {
        let stringified = serde_json::to_string(self)?;

        Ok(Cookie::build((JWT_NAME, stringified))
            .same_site(rocket::http::SameSite::Strict)
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .expires(OffsetDateTime::now_utc().saturating_add(Duration::days(7)))
            .build())
    }

    pub async fn get_uid_by_id(
        db: &mut Connection<DbConn>,
        id: &i64,
    ) -> Result<String, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT uid
                FROM users
                WHERE _id = $1;
            "#,
            id
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.uid.to_string())
    }

    pub async fn is_uid_owned_by_id(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
        id: &i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM users
                    WHERE uid = $1 AND _id = $2
                ) AS exists
            "#,
            uid,
            id
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    pub async fn is_valid(&self, db: &mut Connection<DbConn>) -> Result<bool, sqlx::Error> {
        let Ok(uid) = Uuid::from_str(self.uid.as_str()) else {
            return Ok(false);
        };

        let query_result = sqlx::query!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM users
                    WHERE uid = $1
                ) AS exists
            "#,
            uid
        )
        .fetch_one(&mut ***db)
        .await?;

        Ok(query_result.exists.map_or(false, |s| s))
    }

    pub async fn get_uid_by_display_name(
        db: &mut Connection<DbConn>,
        name: &str,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT uid
                FROM users
                WHERE display_name = $1;
            "#,
            name
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(result.map(|s| s.uid))
    }

    pub async fn get_by_display_name(
        db: &mut Connection<DbConn>,
        name: &str,
    ) -> Result<Option<UserJWT>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT
                uid,
                display_name,
                display_image
                FROM users
                WHERE display_name = $1;
            "#,
            name
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(result.map(|s| UserJWT {
            uid: s.uid.to_string(),
            display_name: s.display_name,
            display_image: s.display_image,
        }))
    }

    pub async fn get_by_uid(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<UserJWT>, sqlx::Error> {
        let result = sqlx::query_as!(
            UserJWT,
            r#"
                SELECT
                uid,
                display_name,
                display_image
                FROM users
                WHERE uid = $1;
            "#,
            uid
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(result)
    }
}
