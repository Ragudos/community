CREATE TABLE IF NOT EXISTS users (
    _id BIGSERIAL PRIMARY KEY,
    _created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    display_name VARCHAR(60) UNIQUE NOT NULL,
    display_image TEXT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS user_metadata (
    _id BIGINT PRIMARY KEY,
    occupation Occupation,
    gender Gender NOT NULL DEFAULT 'unknown',
    biography VARCHAR(255),
    is_private BOOLEAN NOT NULL DEFAULT true,
    account_status AccountStatus NOT NULL DEFAULT 'active',
    FOREIGN KEY (_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_socials (
    _id BIGINT PRIMARY KEY,
    twitter TEXT UNIQUE,
    facebook TEXT UNIQUE,
    instagram TEXT UNIQUE,
    youtube TEXT UNIQUE,
    linkedin TEXT UNIQUE,
    tiktok TEXT UNIQUE,
    reddit TEXT UNIQUE,
    FOREIGN KEY (_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_credentials (
    _id BIGINT PRIMARY KEY,
    email VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(60),
    last_name VARCHAR(60),
    FOREIGN KEY (_id) REFERENCES users(_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_credentials ON user_credentials(_id);
CREATE INDEX IF NOT EXISTS idx_user_socials ON user_socials(_id);
CREATE INDEX IF NOT EXISTS idx_user_metadata ON user_metadata(_id);
CREATE INDEX IF NOT EXISTS idx_users ON users(_id);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(_created_at);
CREATE INDEX IF NOT EXISTS idx_user_name ON user_credentials(first_name, last_name);
CREATE INDEX IF NOT EXISTS idx_user_email ON user_credentials(email);
CREATE INDEX IF NOT EXISTS idx_user_status ON user_metadata(account_status);
CREATE INDEX IF NOT EXISTS idx_user_gender ON user_metadata(gender);
CREATE INDEX IF NOT EXISTS idx_user_occupation ON user_metadata(occupation);
