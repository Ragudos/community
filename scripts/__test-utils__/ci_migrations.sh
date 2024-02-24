#!/bin/bash

# Iterate through each file in the migration folder
for FILE in "/db/sqlx/migrations"/*
do
    if [ -f "$FILE" ]; then
        echo "Running migration: $FILE"
        psql -h $PG_HOST -p $PG_PORT -U $PG_USER -d $PG_DATABAS -f "$FILE"
        if [ $? -eq 0 ]; then
            echo "Migration successful"
        else
            echo "Error: Migration failed for $FILE"
            exit 1
        fi
    fi
done

echo "All migrations completed successfully"

