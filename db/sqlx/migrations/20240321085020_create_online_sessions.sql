CREATE TABLE IF NOT EXISTS online_sessions (
    _user_id BIGINT NOT NULL,
    _updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (_user_id),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE
);