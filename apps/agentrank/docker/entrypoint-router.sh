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
  *)
    echo "AGENTRANK_PROCESS must be searchd, consoled, or agentbot (got: ${AGENTRANK_PROCESS})" >&2
    exit 1
    ;;
esac
