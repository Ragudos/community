CREATE TABLE IF NOT EXISTS followings (
    _follower_id BIGINT NOT NULL,
    _followee_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (_follower_id, _followee_id),
    FOREIGN KEY (_follower_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_followee_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS blocks (
    _user_id BIGINT NOT NULL,
    _blocked_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (_user_id, _blocked_id),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_blocked_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS follow_requests (
    _follower_id BIGINT NOT NULL,
    _followee_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (_follower_id, _followee_id),
    FOREIGN KEY (_follower_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_followee_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_follow_requests_followee ON follow_requests(_followee_id);
CREATE INDEX IF NOT EXISTS idx_follow_requests_follower ON follow_requests(_follower_id);
CREATE INDEX IF NOT EXISTS idx_follow_requests_created_at ON follow_requests(_created_at);

CREATE INDEX IF NOT EXISTS idx_followings_followee ON followings(_followee_id);
CREATE INDEX IF NOT EXISTS idx_followings_follower ON followings(_follower_id);
CREATE INDEX IF NOT EXISTS idx_followings_created_at ON followings(_created_at);

CREATE INDEX IF NOT EXISTS idx_blocks_blocked ON blocks(_blocked_id);
CREATE INDEX IF NOT EXISTS idx_blocks_user ON blocks(_user_id);
CREATE INDEX IF NOT EXISTS idx_blocks_created_at ON blocks(_created_at);
