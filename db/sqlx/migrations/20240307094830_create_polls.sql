CREATE TABLE IF NOT EXISTS polls (
    _id BIGSERIAL PRIMARY KEY,
    _post_id BIGINT NOT NULL,
    question VARCHAR(255) NOT NULL,
    FOREIGN KEY (_post_id) REFERENCES posts(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS poll_options (
    _id BIGSERIAL PRIMARY KEY,
    _poll_id BIGINT NOT NULL,
    content VARCHAR(255) NOT NULL,
    FOREIGN KEY (_poll_id) REFERENCES polls(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS poll_votes (
    _user_id BIGINT NOT NULL,
    _poll_option_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (_user_id, _poll_option_id),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_poll_option_id) REFERENCES poll_options(_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_polls ON polls(_post_id);
CREATE INDEX IF NOT EXISTS idx_poll_options ON poll_options(_poll_id);
CREATE INDEX IF NOT EXISTS idx_poll_votes ON poll_votes(_user_id, _poll_option_id);
CREATE INDEX IF NOT EXISTS idx_poll_votes_created_at ON poll_votes(_created_at);
