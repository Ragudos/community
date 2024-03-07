use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::{
    helpers::db::DbConn,
    models::db::enums::{AccountStatus, Gender, Occupation},
    models::users::schema::FullUserInfo,
};

/// Used when visiting a user's profile page
impl FullUserInfo {
    pub async fn get(
        db: &mut Connection<DbConn>,
        uid: &Uuid,
    ) -> Result<Option<FullUserInfo>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT uid, display_name, display_image,
                occupation as "occupation: Occupation",
                gender as "gender: Gender",
                biography, is_private, account_status as "account_status: AccountStatus",
                facebook, twitter, instagram, linkedin, reddit, tiktok, youtube
                FROM users
                LEFT JOIN user_metadata ON users._id = user_metadata._id
                LEFT JOIN user_socials ON users._id = user_socials._id
                WHERE uid = $1;
            "#,
            uid
        )
        .fetch_optional(&mut ***db)
        .await?;

        Ok(result.map(|s| FullUserInfo {
            uid: s.uid.to_string(),
            display_name: s.display_name,
            display_image: s.display_image,
            occupation: s.occupation,
            account_status: s.account_status,
            gender: s.gender,
            biography: s.biography,
            is_private: s.is_private,
            tiktok: s.tiktok,
            youtube: s.youtube,
            instagram: s.instagram,
            linkedin: s.linkedin,
            reddit: s.reddit,
            twitter: s.twitter,
            facebook: s.facebook,
        }))
    }
}
