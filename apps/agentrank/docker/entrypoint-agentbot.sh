#!/usr/bin/env sh
# AgentBot worker: optional one-shot discover, then run-loop (scale horizontally; shared Redis frontier).
# Migrations: same schema as searchd/consoled; disable if you run sqlx elsewhere (AGENTBOT_BOOT_MIGRATE=0).
set -e
cd /app

if [ "${AGENTBOT_BOOT_MIGRATE:-1}" != "0" ]; then
  echo "agentbot boot: sqlx migrate"
  sqlx migrate run --source migrations
fi

if [ "${AGENTBOT_BOOT_DISCOVER:-0}" = "1" ]; then
  echo "agentbot boot: agentbot discover builtin"
  /usr/local/bin/agentbot discover builtin
fi

echo "agentbot boot: agentbot run-loop"
exec /usr/local/bin/agentbot run-loop
