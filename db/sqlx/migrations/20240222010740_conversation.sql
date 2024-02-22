DO $$
BEGIN
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'conversationtype') THEN
		CREATE TYPE ConversationType as ENUM ('direct', 'group');
	END IF;
END $$;

CREATE TABLE IF NOT EXISTS conversations (
	id SERIAL PRIMARY KEY,
	type ConversationType NOT NULL DEFAULT 'direct',
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS messages (
	id SERIAL PRIMARY KEY,
	conversation_id INTEGER NOT NULL,
	user_id INTEGER NOT NULL,
	content TEXT NOT NULL,
	images TEXT[],
	videos TEXT[],
	links TEXT[],
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS users_conversations (
	user_id INTEGER NOT NULL,
	conversation_id INTEGER NOT NULL,
	PRIMARY KEY (user_id, conversation_id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_conversations ON users_conversations(user_id);
CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id);

