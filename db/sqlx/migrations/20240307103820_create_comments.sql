CREATE TABLE IF NOT EXISTS comments (
    _id BIGSERIAL PRIMARY KEY,
    _post_id BIGINT NOT NULL,
    _user_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _parent_comment_id BIGINT,
    uid UUID NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
    content TEXT NOT NULL,
    images TEXT[],
    videos TEXT[],
    links TEXT[],
    FOREIGN KEY (_post_id) REFERENCES posts(_id) ON DELETE CASCADE,
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_parent_comment_id) REFERENCES comments(_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_comments_post ON comments(_post_id);
CREATE INDEX IF NOT EXISTS idx_comments_user ON comments(_user_id);
CREATE INDEX IF NOT EXISTS idx_comments_created_at ON comments(_created_at);
CREATE INDEX IF NOT EXISTS idx_comments_parent_comment ON comments(_parent_comment_id);
