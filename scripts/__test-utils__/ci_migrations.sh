#!/bin/bash

DIR="db/sqlx/migrations"

# Iterate through each file in the migration folder
for FILE in "$DIR"/*; do
    if [ -f "$FILE" ]; then
        echo "Running migration: $FILE"
        psql -h "localhost" -p "5432" -U "postgres" -d "postgres" -f "$FILE"
        if [ $? -eq 0 ]; then
            echo "Migration successful"
        else
            echo "Error: Migration failed for $FILE"
            exit 1
        fi
    fi
done

echo "All migrations completed successfully"

