use rocket_db_pools::Connection;

use crate::helpers::db::DbConn;
use crate::models::users::schema::UserNameAndImage;

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
        id: &i64,
    ) -> Result<UserNameAndImage, sqlx::Error> {
        Ok(sqlx::query_as!(
            UserNameAndImage,
            r#"
                SELECT display_name, display_image
                FROM users
                WHERE _id = $1
                "#,
            id
        )
        .fetch_one(&mut ***db)
        .await?)
    }
}
