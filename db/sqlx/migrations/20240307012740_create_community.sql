CREATE TABLE IF NOT EXISTS communities (
    _id BIGSERIAL PRIMARY KEY,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    display_name VARCHAR(60) UNIQUE NOT NULL,
    description VARCHAR(255) NOT NULL,
    categories CommunityCategory[] CHECK (array_length(categories, 1) < 4),
    display_image TEXT,
    cover_image TEXT,
    owner_id BIGINT NOT NULL,
    is_private BOOLEAN NOT NULL DEFAULT true,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (owner_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS community_memberships (
    _user_id BIGINT NOT NULL,
    _community_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    role UserRole NOT NULL DEFAULT 'user',
    PRIMARY KEY (_user_id, _community_id),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_community_id) REFERENCES communities(_id) ON DELETE CASCADE
);

-- For private communities
CREATE TABLE IF NOT EXISTS community_join_requests (
    _community_id BIGINT NOT NULL,
    _user_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reason VARCHAR(255) NOT NULL,
    PRIMARY KEY (_community_id, _user_id),
    FOREIGN KEY (_community_id) REFERENCES communities(_id) ON DELETE CASCADE,
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS community_posts (
    _post_id BIGINT NOT NULL,
    _community_id BIGINT NOT NULL,
    PRIMARY KEY (_post_id, _community_id),
    FOREIGN KEY (_post_id) REFERENCES posts(_id) ON DELETE CASCADE,
    FOREIGN KEY (_community_id) REFERENCES communities(_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_community_posts ON community_posts(_community_id, _post_id);
CREATE INDEX IF NOT EXISTS idx_community_join_requests ON community_join_requests(_community_id, _user_id);
CREATE INDEX IF NOT EXISTS idx_community_join_requests_created_at ON community_join_requests(_created_at);

CREATE INDEX IF NOT EXISTS idx_community_memberships ON community_memberships(_community_id, _user_id);
CREATE INDEX IF NOT EXISTS idx_community_memberships_role ON community_memberships(role);
CREATE INDEX IF NOT EXISTS idx_community_memberships_created_at ON community_memberships(_created_at);

CREATE INDEX IF NOT EXISTS idx_communities_owner_id ON communities(owner_id);
