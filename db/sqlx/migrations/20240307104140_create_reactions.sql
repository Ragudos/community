CREATE TABLE IF NOT EXISTS post_reactions (
    _user_id BIGINT NOT NULL,
    _post_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reaction VARCHAR(255) NOT NULL,
    PRIMARY KEY (_user_id, _post_id),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_post_id) REFERENCES posts(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS comment_reactions (
    _user_id BIGINT NOT NULL,
    _comment_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reaction VARCHAR(255) NOT NULL,
    PRIMARY KEY (_user_id, _comment_id),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_comment_id) REFERENCES comments(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS message_reactions (
    _user_id BIGINT NOT NULL,
    _message_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reaction VARCHAR(255) NOT NULL,
    PRIMARY KEY (_user_id, _message_id),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_message_id) REFERENCES messages(_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_post_reactions_post ON post_reactions(_post_id);
CREATE INDEX IF NOT EXISTS idx_post_reactions_user ON post_reactions(_user_id);
CREATE INDEX IF NOT EXISTS idx_post_reactions_created_at ON post_reactions(_created_at);

CREATE INDEX IF NOT EXISTS idx_comment_reactions_comment ON comment_reactions(_comment_id);
CREATE INDEX IF NOT EXISTS idx_comment_reactions_user ON comment_reactions(_user_id);
CREATE INDEX IF NOT EXISTS idx_comment_reactions_created_at ON comment_reactions(_created_at);

CREATE INDEX IF NOT EXISTS idx_message_reactions_message ON message_reactions(_message_id);
CREATE INDEX IF NOT EXISTS idx_message_reactions_user ON message_reactions(_user_id);
CREATE INDEX IF NOT EXISTS idx_message_reactions_created_at ON message_reactions(_created_at);
