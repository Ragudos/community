DO $$
BEGIN
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'gender') THEN
		CREATE TYPE Gender as ENUM ('male', 'female', 'other', 'notspecified');
	END IF;
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'occupation') THEN
		CREATE TYPE Occupation as ENUM ('student', 'teacher', 'engineer', 'doctor', 'lawyer', 'unemployed', 'other');
	END IF;
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'userrole') THEN
		CREATE TYPE UserRole as ENUM ('owner', 'admin', 'moderator', 'user');
	END IF;
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'referrals') THEN
		CREATE TYPE Referrals as ENUM ('youtube', 'facebook', 'instagram', 'twitter', 'tiktok', 'reddit', 'linkedin', 'other');
	END IF;
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'requeststatus') THEN
		CREATE TYPE RequestStatus as ENUM ('accepted', 'pending', 'rejected', 'blocked');
	END IF;
END $$;
		

CREATE TABLE IF NOT EXISTS users (
	id SERIAL PRIMARY KEY,
	display_name VARCHAR(40) UNIQUE NOT NULL,
	display_image TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS users_metadata (
	id INTEGER PRIMARY KEY,
	occupation Occupation NOT NULL,
	gender Gender NOT NULL,
	biography VARCHAR(255),
	is_private BOOLEAN NOT NULL,
	last_login_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users_socials (
	id INTEGER PRIMARY KEY,
	twitter TEXT UNIQUE,
	facebook TEXT UNIQUE,
	instagram TEXT UNIQUE,
	youtube TEXT UNIQUE,
	linkedin TEXT UNIQUE,
	tiktok TEXT UNIQUE,
	reddit TEXT UNIQUE,
	FOREIGN KEY (id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users_credentials (
	id INTEGER PRIMARY KEY,
	email VARCHAR(100) UNIQUE NOT NULL,
	password_hash TEXT NOT NULL,
	first_name VARCHAR(60) NOT NULL,
	last_name VARCHAR(60) NOT NULL,
	FOREIGN KEY (id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS communities (
	id SERIAL PRIMARY KEY,
	display_name VARCHAR(50) UNIQUE NOT NULL,
	display_image TEXT NOT NULL,
	cover_image TEXT,
	description VARCHAR(255) NOT NULL,
	owner_id INTEGER NOT NULL,
	is_private BOOLEAN NOT NULL,
	category VARCHAR(60),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS community_memberships (
	user_id INTEGER NOT NULL,
	community_id INTEGER NOT NULL,
	joined_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	role UserRole NOT NULL DEFAULT 'user',
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS posts (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL,
	community_id INTEGER,
	content TEXT NOT NULL,
	caption TEXT,
	links TEXT[],
	images TEXT[],
	videos TEXT[],
	is_pinned BOOLEAN NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS comments (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL,
	post_id INTEGER NOT NULL,
	content TEXT NOT NULL,
	links TEXT[],
	images TEXT[],
	videos TEXT[],
	parent_comment_id INTEGER,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (parent_comment_id) REFERENCES comments(id) ON DELETE CASCADE,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

-- For private communities
CREATE TABLE IF NOT EXISTS community_join_requests (
	id SERIAL PRIMARY KEY,
	community_id INTEGER NOT NULL,
	user_id INTEGER NOT NULL,
	reason VARCHAR(255) NOT NULL,
	known_from Referrals NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	status RequestStatus NOT NULL DEFAULT 'pending',
	FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users_posts (
	user_id INTEGER NOT NULL,
	post_id INTEGER NOT NULL,
	PRIMARY KEY (user_id, post_id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS community_posts (
	post_id INTEGER NOT NULL,
	community_id INTEGER NOT NULL,
	PRIMARY KEY (post_id, community_id),
	FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
	FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users_comments (
	user_id INTEGER NOT NULL,
	comment_id INTEGER NOT NULL,
	post_id INTEGER NOT NULL,
	PRIMARY KEY (user_id, post_id, comment_id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE,
	FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users_token (
	user_id INTEGER NOT NULL PRIMARY KEY,
	refresh_token TEXT NOT NULL,
	refresh_token_expires_in TIMESTAMP WITH TIME ZONE NOT NULL,
	refresh_token_creation_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users_owned_communities(
	user_id INTEGER NOT NULL,
	community_id INTEGER NOT NULL,
	PRIMARY KEY (user_id, community_id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_users_owned_communities ON users_owned_communities(user_id);

-- SELECT * FROM community_posts WHERE community_id = $1; Get all posts in a community.
CREATE INDEX IF NOT EXISTS idx_community_posts ON community_posts(community_id);

-- SELECT * FROM community_memberships WHERE community_id = $1 AND role = $2; Get all members in a community with specific roles.
CREATE INDEX IF NOT EXISTS idx_community_memberships_all_users_with_role ON community_memberships(user_id, role);

-- SELECT * FROM community_memberships WHERE user_id = $1; Get all communites a user joined in.
CREATE INDEX IF NOT EXISTS idx_community_memberships_of_user ON community_memberships(user_id);

-- SELECT * FROM users WHERE occupation = $1; Get all users of an occupation
CREATE INDEX IF NOT EXISTS idx_occupation ON users_metadata(occupation);

-- SELECT * FROM users WHERE gender = $1; Get all users of a gender
CREATE INDEX IF NOT EXISTS idx_gender ON users_metadata(gender);

-- SELECT * FROM posts WHERE community_id = $1 AND is_pinned = $2; Get all pinned posts
-- in a community to show it at the top of a community page.
CREATE INDEX IF NOT EXISTS idx_community_post_is_pinned ON posts(community_id, is_pinned);

-- SELECT * FROM users_posts WHERE user_id = $1; Get all posts of a user
CREATE INDEX IF NOT EXISTS idx_user_posts ON users_posts(user_id);

-- SELECT * FROM users_posts WHERE user_id = $1; Get all comments of a user
CREATE INDEX IF NOT EXISTS idx_user_comments ON users_comments(user_id);

-- SELECT * FROM comments WHERE post_id = $1; Get all comments of a post
CREATE INDEX IF NOT EXISTS idx_post_comments ON comments(post_id);

-- SELECT * FROM comments WHERE post_id = $1 AND user_id = $2; Get all comments of a user in a post
CREATE INDEX IF NOT EXISTS idx_comments_of_user_in_post ON comments(post_id, user_id);

-- SELECT * FROM comments WHERE parent_comment_id = $1; Get replies of a comment/reply
CREATE INDEX IF NOT EXISTS idx_replies_in_comment ON comments(parent_comment_id);


