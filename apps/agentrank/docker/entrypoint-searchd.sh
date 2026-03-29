#!/usr/bin/env sh
# Production entrypoint: apply DB migrations, rebuild Tantivy index (optional), start searchd.
# Disable steps with SEARCHD_BOOT_MIGRATE=0 or SEARCHD_BOOT_REBUILD_INDEX=0 if you run them in a Railway Release Command instead.
# SEARCHD_INDEX_BOOT=full (default): always rebuild when rebuild is enabled.
# SEARCHD_INDEX_BOOT=reuse: skip rebuild if `agentrank-index probe` succeeds (use with a persistent volume).
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
  BOOT="${SEARCHD_INDEX_BOOT:-full}"
  if [ "$BOOT" = "reuse" ] && agentrank-index probe --output "$SEARCH_INDEX_PATH"; then
    echo "searchd boot: index reuse (probe ok) -> $SEARCH_INDEX_PATH"
  else
    echo "searchd boot: index rebuild -> $SEARCH_INDEX_PATH (BOOT=$BOOT)"
    agentrank-index rebuild --output "$SEARCH_INDEX_PATH"
  fi
fi

echo "searchd boot: exec searchd"
exec searchd
