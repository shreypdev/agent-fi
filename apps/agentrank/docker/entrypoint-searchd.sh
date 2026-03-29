#!/usr/bin/env sh
# Production entrypoint: apply DB migrations, rebuild Tantivy index, start searchd.
# Disable steps with SEARCHD_BOOT_MIGRATE=0 or SEARCHD_BOOT_REBUILD_INDEX=0 if you run them in a Railway Release Command instead.
set -e
cd /app

SEARCH_INDEX_PATH="${SEARCH_INDEX_PATH:-/tmp/agentrank-index}"
export SEARCH_INDEX_PATH
mkdir -p "$SEARCH_INDEX_PATH"

if [ "${SEARCHD_BOOT_MIGRATE:-1}" != "0" ]; then
  echo "searchd boot: sqlx migrate"
  sqlx migrate run --source migrations
fi

if [ "${SEARCHD_BOOT_REBUILD_INDEX:-1}" != "0" ]; then
  echo "searchd boot: index rebuild -> $SEARCH_INDEX_PATH"
  agentrank-index rebuild --output "$SEARCH_INDEX_PATH"
fi

echo "searchd boot: exec searchd"
exec searchd
