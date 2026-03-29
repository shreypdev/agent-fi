#!/usr/bin/env bash
# Phase 0 integration gate: seed frontier → drain → rebuild index → smoke search.
# Requires: DATABASE_URL, REDIS_URL, SEARCH_INDEX_PATH, built binaries (cargo build),
# and a seed file (default: tests/fixtures/phase0_seed_urls.txt).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SEED="${PHASE0_SEED_FILE:-$ROOT/tests/fixtures/phase0_seed_urls.txt}"
DRAIN_MAX="${PHASE0_DRAIN_MAX:-50}"
: "${DATABASE_URL:?}"
: "${REDIS_URL:?}"
: "${SEARCH_INDEX_PATH:?}"

echo "== Phase0 gate: enqueue from $SEED"
while IFS= read -r line || [[ -n "$line" ]]; do
  [[ -z "${line// }" ]] && continue
  [[ "$line" =~ ^# ]] && continue
  cargo run --quiet -p agentrank-agentbot --bin agentbot -- enqueue "$line" --priority 1.0
done <"$SEED"

echo "== Phase0 gate: drain (max $DRAIN_MAX)"
cargo run --quiet -p agentrank-agentbot --bin agentbot -- drain --max "$DRAIN_MAX"

echo "== Phase0 gate: index rebuild"
cargo run --quiet -p agentrank-search-index --bin agentrank-index -- rebuild --output "$SEARCH_INDEX_PATH"

SEARCHD_URL="${SEARCHD_URL:-http://127.0.0.1:8080}"
echo "== Phase0 gate: POST $SEARCHD_URL/v1/search (expect 200)"
code="$(curl -sS -o /tmp/phase0_search.json -w '%{http_code}' \
  -X POST "$SEARCHD_URL/v1/search" \
  -H 'content-type: application/json' \
  -d '{"query":"demo","limit":5,"offset":0}')"
if [[ "$code" != "200" ]]; then
  echo "search failed: HTTP $code" >&2
  cat /tmp/phase0_search.json >&2 || true
  exit 1
fi
echo "OK (HTTP 200). Response saved to /tmp/phase0_search.json"
