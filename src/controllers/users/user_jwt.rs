use rocket::http::{Cookie, SameSite};
use rocket_db_pools::Connection;
use time::{Duration, OffsetDateTime};

use crate::helpers::db::DbConn;
use crate::models::users::schema::UserJWT;
use crate::models::JWT_NAME;

impl UserJWT {
    pub fn to_cookie(&self) -> Result<Cookie<'static>, serde_json::Error> {
        let stringified = serde_json::to_string(self)?;

        Ok(Cookie::build((JWT_NAME, stringified))
            .same_site(rocket::http::SameSite::Strict)
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .expires(
                OffsetDateTime::now_utc().saturating_add(Duration::days(7)),
            )
            .build())
    }

    pub async fn verify(
        &self,
        db: &mut Connection<DbConn>,
    ) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                    SELECT
                    EXISTS(
                        SELECT 1
                        FROM users
                        WHERE _id = $1
                    ) AS exists;
                "#,
            self._id,
        )
        .fetch_one(&mut ***db)
        .await?
        .exists
        .unwrap_or(false))
    }

    #[allow(non_snake_case)]
    pub async fn get_by_display_name(
        db: &mut Connection<DbConn>,
        name: &str,
    ) -> Result<Option<UserJWT>, sqlx::Error> {
        Ok(sqlx::query_as!(
            UserJWT,
            r#"
                    SELECT
                    _id,
                    display_name,
                    display_image
                    FROM users
                    WHERE display_name = $1;
                "#,
            name
        )
        .fetch_optional(&mut ***db)
        .await?)
    }

    #[allow(non_snake_case)]
    pub async fn get_by_id(
        db: &mut Connection<DbConn>,
        id: &i64,
    ) -> Result<Option<UserJWT>, sqlx::Error> {
        Ok(sqlx::query_as!(
            UserJWT,
            r#"
                    SELECT
                    _id,
                    display_name,
                    display_image
                    FROM users
                    WHERE _id = $1;
                "#,
            id
        )
        .fetch_optional(&mut ***db)
        .await?)
    }
}
