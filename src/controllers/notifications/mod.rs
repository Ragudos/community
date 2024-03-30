use rocket_db_pools::Connection;
use sqlx::{Postgres, Transaction};

use crate::helpers::db::DbConn;
use crate::models::db::enums::NotificationType;
use crate::models::notifications::{Notification, UserNotificationPreference};

impl UserNotificationPreference {
    /// Enables or disables a preference for a notification type.
    /// On conflict, we create the preference.
    pub async fn enable_disable_preference(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i64,
        notification_type: NotificationType,
        enabled: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_notification_preferences (
                _user_id,
                notification_type,
                enabled
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (_user_id, notification_type) DO UPDATE
            SET enabled = $3;
            "#,
            user_id,
            notification_type as NotificationType,
            enabled,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Returns false if the user does not have a preference for the notification type or its disabled.
    /// By default, a non-existent preference is considered disabled.
    /// Returns true if the user has a preference for the notification type and its enabled.
    pub async fn does_user_prefer_notification_type(
        conn: &mut Connection<DbConn>,
        user_id: &i64,
        notification_type: NotificationType,
    ) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
            SELECT enabled
            FROM user_notification_preferences
            WHERE _user_id = $1
            AND notification_type = $2
            "#,
            user_id,
            notification_type as NotificationType,
        )
        .fetch_optional(&mut ***conn)
        .await?
        .map_or(false, |row| row.enabled))
    }

    #[allow(non_snake_case)]
    pub async fn get_all_preferences_of_user(
        conn: &mut Connection<DbConn>,
        user_id: &i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
            UserNotificationPreference,
            r#"
            SELECT
            _user_id,
            notification_type AS "notification_type: NotificationType",
            enabled
            FROM user_notification_preferences
            WHERE _user_id = $1
            "#,
            user_id,
        )
        .fetch_all(&mut ***conn)
        .await?)
    }
}

impl Notification {
    #[allow(non_snake_case)]
    pub async fn get_all_read_notifications_of_user(
        conn: &mut Connection<DbConn>,
        user_id: &i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
            Notification,
            r#"
            SELECT
            _id,
            _recipient_id,
            _sender_id,
            _created_at,
            notification_type AS "notification_type: NotificationType",
            is_read,
            message,
            link
            FROM notifications
            WHERE _recipient_id = $1
            AND is_read = true
            ORDER BY _created_at DESC
            "#,
            user_id,
        )
        .fetch_all(&mut ***conn)
        .await?)
    }

    #[allow(non_snake_case)]
    pub async fn get_all_unread_notifications_of_user(
        conn: &mut Connection<DbConn>,
        user_id: &i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
            Notification,
            r#"
            SELECT
            _id,
            _recipient_id,
            _sender_id,
            _created_at,
            notification_type AS "notification_type: NotificationType",
            is_read,
            message,
            link
            FROM notifications
            WHERE _recipient_id = $1
            AND is_read = false
            ORDER BY _created_at DESC
            "#,
            user_id,
        )
        .fetch_all(&mut ***conn)
        .await?)
    }

    #[allow(non_snake_case)]
    pub async fn get_all_notifications_of_user(
        conn: &mut Connection<DbConn>,
        user_id: &i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
            Notification,
            r#"
            SELECT
            _id,
            _recipient_id,
            _sender_id,
            _created_at,
            notification_type AS "notification_type: NotificationType",
            is_read,
            message,
            link
            FROM notifications
            WHERE _recipient_id = $1
            ORDER BY _created_at DESC
            "#,
            user_id,
        )
        .fetch_all(&mut ***conn)
        .await?)
    }

    pub async fn delete_all_notifications_of_user(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM notifications
            WHERE _recipient_id = $1
            "#,
            user_id,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn delete_all_read_notifications_of_user(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM notifications
            WHERE _recipient_id = $1
            AND is_read = true
            "#,
            user_id,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn mark_all_notifications_of_user_as_read(
        tx: &mut Transaction<'_, Postgres>,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE notifications
            SET is_read = true
            WHERE _recipient_id = $1
            "#,
            user_id,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn mark_as_read(
        tx: &mut Transaction<'_, Postgres>,
        notification_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE notifications
            SET is_read = true
            WHERE _id = $1
            "#,
            notification_id,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    #[allow(non_snake_case)]
    pub async fn create(
        tx: &mut Transaction<'_, Postgres>,
        recipient_id: &i64,
        sender_id: &i64,
        notification_type: NotificationType,
        message: &str,
        link: Option<&str>
    ) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as!(
            Notification,
            r#"
            INSERT INTO notifications (_recipient_id, _sender_id, notification_type, message, link)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
            _id,
            _recipient_id,
            _sender_id,
            _created_at,
            notification_type AS "notification_type: NotificationType",
            is_read,
            message,
            link
            "#,
            recipient_id,
            sender_id,
            notification_type as NotificationType,
            message,
            link
        )
        .fetch_one(&mut **tx)
        .await?)
    }
}
