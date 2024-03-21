use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::{helpers::db::DbConn, models::users::schema::UserNameAndImage};

pub mod full_user_info;
pub mod preferences;
pub mod request_guard;
pub mod user_credentials;
pub mod user_jwt;
pub mod user_metadata;
pub mod user_table;

impl UserNameAndImage {
    pub async fn get(
        db: &mut Connection<DbConn>,
        uid: &Uuid
    ) -> Result<UserNameAndImage, sqlx::Error> {
        Ok(
            sqlx::query_as!(
                UserNameAndImage,
                r#"
                SELECT display_name, display_image
                FROM users
                WHERE uid = $1
                "#,
                uid
            ).fetch_one(&mut ***db).await?
        )
    }
}
