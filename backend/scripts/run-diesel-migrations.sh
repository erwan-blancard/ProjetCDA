#!/usr/bin/env bash
set -e

echo "Running diesel migration scripts..."

find /docker-entrypoint-initdb.d/migrations/**/up.sql -exec \
    psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" -f {} \;