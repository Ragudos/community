DO $$
BEGIN
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'reaction') THEN
		CREATE TYPE Reaction AS ENUM ('like', 'dislike', 'love', 'haha', 'wow', 'sad', 'angry');
	END IF;
END$$;

ALTER TABLE IF EXISTS posts ADD COLUMN IF NOT EXISTS has_poll BOOLEAN DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS polls (
	id SERIAL PRIMARY KEY,
	post_id INTEGER NOT NULL,
	question TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS poll_options (
	id SERIAL PRIMARY KEY,
	poll_id INTEGER NOT NULL,
	content VARCHAR(255) NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (poll_id) REFERENCES polls(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS poll_votes (
	user_id INTEGER NOT NULL,
	poll_option_id INTEGER NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (user_id, poll_option_id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (poll_option_id) REFERENCES poll_options(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS followings (
	follower_id INTEGER NOT NULL,
	followee_id INTEGER NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (follower_id, followee_id),
	FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (followee_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS blocks (
	user_id INTEGER NOT NULL,
	blocked_id INTEGER NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (user_id, blocked_id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (blocked_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS reactions (
	user_id INTEGER NOT NULL,
	post_id INTEGER,
	comment_id INTEGER,
	type Reaction NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (user_id, post_id, comment_id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
	FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_all_followees_of_user ON followings(follower_id);
CREATE INDEX IF NOT EXISTS idx_all_followers_of_user ON followings(followee_id);

CREATE INDEX IF NOT EXISTS idx_polls_on_post ON polls(post_id);
CREATE INDEX IF NOT EXISTS idx_poll_options_on_poll ON poll_options(poll_id);
CREATE INDEX IF NOT EXISTS idx_poll_votes_on_user ON poll_votes(user_id);
CREATE INDEX IF NOT EXISTS idx_poll_votes_on_poll_option ON poll_votes(poll_option_id);

CREATE INDEX IF NOT EXISTS idx_reactions_on_post ON reactions(post_id);
CREATE INDEX IF NOT EXISTS idx_reactions_on_comment ON reactions(comment_id);
CREATE INDEX IF NOT EXISTS idx_reactions_of_user ON reactions(user_id);

