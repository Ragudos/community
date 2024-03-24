use rocket_db_pools::Connection;

use crate::{
    helpers::db::DbConn,
    models::db::enums::{AccountStatus, Gender, Occupation},
    models::users::schema::FullUserInfo,
};

/// Used when visiting a user's profile page
impl FullUserInfo {
    #[allow(non_snake_case)]
    pub async fn get(db: &mut Connection<DbConn>, id: &i64) -> Result<FullUserInfo, sqlx::Error> {
        Ok(sqlx::query_as!(
            FullUserInfo,
            r#"
                    SELECT users._id, display_name, display_image,
                    occupation as "occupation: Occupation",
                    gender as "gender: Gender",
                    biography, is_private, account_status as "account_status: AccountStatus",
                    facebook, twitter, instagram, linkedin, reddit, tiktok, youtube
                    FROM users
                    LEFT JOIN user_metadata ON users._id = user_metadata._id
                    LEFT JOIN user_socials ON users._id = user_socials._id
                    WHERE users._id = $1;
                "#,
            id
        )
        .fetch_one(&mut ***db)
        .await?)
    }
}
