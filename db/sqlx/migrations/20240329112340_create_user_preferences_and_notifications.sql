DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type
        WHERE typname = 'notificationtype'
        AND typnamespace = (
            SELECT oid FROM pg_namespace
            WHERE nspname = 'public'
        )
    ) THEN
        -- Notification types
        CREATE TYPE NotificationType AS ENUM (
            -- For follows (follow request, accept/reject, follow/unfollow)
            'follows',
            -- For community entrace (join request, accept/reject, join/leave)
            'communityentrance',
            -- For community posts (new post, comment, like)
            'communityposts'
        );
    END IF;
END $$;

-- Will not be created unless a user turns on notifications
-- for a specific type
CREATE TABLE IF NOT EXISTS user_notification_preferences (
    _user_id BIGINT NOT NULL,
    notification_type NotificationType NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (_user_id, notification_type),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS notifications (
    _id BIGSERIAL PRIMARY KEY,
    _recipient_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    notification_type NotificationType NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    message TEXT NOT NULL,
    FOREIGN KEY (_recipient_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_notification_preferences__user_id ON user_notification_preferences (_user_id);
CREATE INDEX IF NOT EXISTS idx_user_notification_preferences__notification_type ON user_notification_preferences (notification_type);

CREATE INDEX IF NOT EXISTS idx_notifications__recipient_id ON notifications (_recipient_id);
CREATE INDEX IF NOT EXISTS idx_notifications__notification_type ON notifications (notification_type);
CREATE INDEX IF NOT EXISTS idx_notifications__created_at ON notifications (_created_at);