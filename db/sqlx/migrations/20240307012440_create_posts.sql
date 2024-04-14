CREATE TABLE IF NOT EXISTS posts (
    _id BIGSERIAL PRIMARY KEY,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    owner_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    caption VARCHAR(255),
    images TEXT[],
    videos TEXT[],
    is_pinned BOOLEAN NOT NULL DEFAULT false,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (owner_id) REFERENCES users(_id) ON DELETE CASCADE
);
