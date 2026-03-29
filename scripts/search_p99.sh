#!/usr/bin/env bash
# Latency smoke for searchd: prints min / p50 / p95 / p99 (ms) for N POSTs to /v1/search.
# Dev expectation: P99 < 200ms on reference hardware with a warm index (see apps/agentrank README).
set -euo pipefail
BASE="${SEARCHD_URL:-http://127.0.0.1:8080}"
N="${1:-50}"
QUERY="${2:-portfolio}"
body=$(printf '{"query":"%s","limit":10}' "$QUERY")
times=()
for ((i = 0; i < N; i++)); do
  ms=$(curl -s -o /dev/null -w "%{time_total}" -X POST "$BASE/v1/search" \
    -H "content-type: application/json" -d "$body" | awk '{printf "%.0f\n", $1*1000}')
  times+=("$ms")
done
IFS=$'\n' sorted=($(printf '%s\n' "${times[@]}" | sort -n))
len=${#sorted[@]}
idx() {
  local p="$1"
  local x=$(( (len * p + 99) / 100 ))
  if (( x >= len )); then x=$((len - 1)); fi
  echo "${sorted[$x]}"
}
echo "n=$N query=$QUERY"
echo "min=${sorted[0]}ms p50=$(idx 50)ms p95=$(idx 95)ms p99=$(idx 99)ms max=${sorted[$((len-1))]}ms"
