#!/bin/bash

## Set PostgreSQL connection details
PG_HOST = "localhost"
PG_PORT = "5432"
PG_USER="your_username"
PG_PASSWORD="your_password"
PG_DATABASE="your_database"

# Set the path to your migration folder
MIGRATION_FOLDER="/db/sqlx/migrations"

# Iterate through each file in the migration folder
for FILE in "$MIGRATION_FOLDER"/*
do
    if [ -f "$FILE" ]; then
        echo "Running migration: $FILE"
        psql -h $PG_HOST -p $PG_PORT -U $PG_USER -d $PG_DATABASE -f "$FILE"
        if [ $? -eq 0 ]; then
            echo "Migration successful"
        else
            echo "Error: Migration failed for $FILE"
            exit 1
        fi
    fi
done

echo "All migrations completed successfully"

