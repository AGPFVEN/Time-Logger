#!/bin/bash

# [IA GENERATED]

if [ -z "$1" ]; then
    echo "Error: migration name not received."
    exit 1
fi

# 1. Create base folders if they doesn't exist
mkdir -p migrations/sqlite
mkdir -p migrations/postgres

# 2. Generate SQLite migration
diesel migration generate "$1" --migration-dir migrations/sqlite

# 3. Capture name of migration
MIGRATION_DIR_SQLITE=$(ls -td migrations/sqlite/*_"$1" | head -1)

if [ -z "$MIGRATION_DIR_SQLITE" ]; then
    echo "Error: Migration name couldn't be found."
    exit 1
fi

# 4. Extract dir name
FOLDER_NAME=$(basename "$MIGRATION_DIR_SQLITE")

# 5. Replicate for postgres
MIGRATION_DIR_POSTGRES="migrations/postgres/$FOLDER_NAME"
mkdir -p "$MIGRATION_DIR_POSTGRES"

# 6. Create empty folders for Postgres
touch "$MIGRATION_DIR_POSTGRES/up.sql" "$MIGRATION_DIR_POSTGRES/down.sql"

echo "Dual migration created successfully:"
echo " - $MIGRATION_DIR_SQLITE"
echo " - $MIGRATION_DIR_POSTGRES"