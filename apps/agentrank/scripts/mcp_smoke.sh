#!/usr/bin/env sh
# Smoke-test MCP JSON-RPC against searchd (requires running searchd).
set -e
BASE="${1:-http://127.0.0.1:8080}"
curl -sf -X POST "$BASE/mcp" -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | head -c 400
echo
curl -sf "$BASE/.well-known/mcp.json" | head -c 400
echo
