CREATE TABLE IF NOT EXISTS conversations (
    _id BIGSERIAL PRIMARY KEY,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    type ConversationType NOT NULL DEFAULT 'direct'
);

CREATE TABLE IF NOT EXISTS messages (
    _id BIGSERIAL PRIMARY KEY,
    _conversation_id BIGINT NOT NULL,
    _user_id BIGINT NOT NULL,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    images TEXT[],
    videos TEXT[],
    links TEXT[],
    FOREIGN KEY (_conversation_id) REFERENCES conversations(_id) ON DELETE CASCADE,
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_conversations (
    _user_id BIGINT NOT NULL,
    _conversation_id BIGINT NOT NULL,
    PRIMARY KEY (_user_id, _conversation_id),
    FOREIGN KEY (_user_id) REFERENCES users(_id) ON DELETE CASCADE,
    FOREIGN KEY (_conversation_id) REFERENCES conversations(_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_conversations ON user_conversations(_user_id);
CREATE INDEX IF NOT EXISTS idx_messages_conversation ON user_conversations(_conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(_conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(_created_at);
CREATE INDEX IF NOT EXISTS idx_conversations_created_at ON conversations(_created_at);
