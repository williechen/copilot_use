#!/usr/bin/env bash
set -euo pipefail
# The container bootstrap owner is never an application login.
psql --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
  --set ON_ERROR_STOP=1 --set app_user="$PG_APP_USER" \
  --set app_password="$PG_APP_PASSWORD" --set app_db="$POSTGRES_DB" <<'SQL'
CREATE ROLE :"app_user" LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE PASSWORD :'app_password';
ALTER DATABASE :"app_db" OWNER TO :"app_user";
REVOKE CONNECT ON DATABASE postgres FROM PUBLIC;
SQL
if [ "${PG_ENABLE_VECTOR:-false}" = true ]; then
  psql --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" --set ON_ERROR_STOP=1 -c 'CREATE EXTENSION vector'
fi
