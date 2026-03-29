#!/usr/bin/env sh
# Routes one AgentRank image to searchd | consoled | agentbot (Railway: set AGENTRANK_PROCESS per service).
set -e
case "${AGENTRANK_PROCESS:-searchd}" in
  searchd)
    exec /entrypoint-searchd.sh
    ;;
  consoled)
    exec /usr/local/bin/consoled
    ;;
  agentbot)
    exec /entrypoint-agentbot.sh
    ;;
  healthd)
    exec /usr/local/bin/healthd
    ;;
  *)
    echo "AGENTRANK_PROCESS must be searchd, consoled, agentbot, or healthd (got: ${AGENTRANK_PROCESS})" >&2
    exit 1
    ;;
esac
