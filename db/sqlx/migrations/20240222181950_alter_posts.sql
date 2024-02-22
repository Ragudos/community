DO $$
BEGIN
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'visibility') THEN
		CREATE TYPE Visibility as ENUM ('public', 'followers', 'private');
	END IF;
END $$;

ALTER TABLE IF EXISTS posts ADD COLUMN visibility Visibility DEFAULT 'public';

