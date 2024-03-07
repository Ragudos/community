DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type
        WHERE typname = 'gender'
        AND typnamespace = (
            SELECT oid FROM pg_namespace
            WHERE nspname = 'public'
        )
    ) THEN
        CREATE TYPE Gender AS ENUM (
            'male',
            'female',
            'other',
            'unknown'
        );
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM pg_type
        WHERE typname = 'occupation'
        AND typnamespace = (
            SELECT oid FROM pg_namespace
            WHERE nspname = 'public'
        )
    ) THEN
        CREATE TYPE Occupation AS ENUM (
            'student',
            'teacher',
            'engineer',
            'doctor',
            'lawyer',
            'developer',
            'artist',
            'unemployed',
            'other'
        );
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM pg_type
        WHERE typname = 'userrole'
        AND typnamespace = (
            SELECT oid FROM pg_namespace
            WHERE nspname = 'public'
        )
    ) THEN
        CREATE TYPE UserRole AS ENUM (
            'admin',
            'moderator',
            'user'
        );
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM pg_type
        WHERE typname = 'accountstatus'
        AND typnamespace = (
            SELECT oid FROM pg_namespace
            WHERE nspname = 'public'
        )
    ) THEN
        CREATE TYPE AccountStatus AS ENUM (
            'active',
            'deactivated',
            'banned'
        );
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM pg_type
        WHERE typname = 'communitycategory'
        AND typnamespace = (
            SELECT oid FROM pg_namespace
            WHERE nspname = 'public'
        )
    ) THEN
        CREATE TYPE CommunityCategory AS ENUM (
            'art',
            'music',
            'gaming',
            'sports',
            'science',
            'technology',
            'literature',
            'healthandfitness',
            'selfimprovement',
            'academics',
            'other'
        );
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM pg_type
        WHERE typname = 'conversationtype'
        AND typnamespace = (
            SELECT oid FROM pg_namespace
            WHERE nspname = 'public'
        )
    ) THEN
        CREATE TYPE ConversationType AS ENUM (
            'direct',
            'group'
        );
    END IF;
END $$;