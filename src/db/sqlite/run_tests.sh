#!/usr/bin/env bash

DB="testing.db"

if [ -f "$DB" ]; then
    rm "$DB"
fi

echo -e "\e[34m=== INITIALIZING SQLITE ENVIRONMENT ===\e[0m"

for script in *.sql; do
    echo -e "\n\e[33m[>] Running: $script\e[0m"
    
    sqlite3 -cmd ".mode box" "$DB" ".read $script"

done

echo -e "\n\e[32m[OK] Tests completed.\e[0m"

rm testing.db
