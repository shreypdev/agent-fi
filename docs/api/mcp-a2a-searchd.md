# AgentRank searchd — MCP and structured A2A

All endpoints are served by **`searchd`** (same base URL as `POST /v1/search`). Set **`AGENTRANK_PUBLIC_URL`** in production so `/.well-known/*` manifests resolve correctly.

## MCP (Streamable HTTP)

| URL | Purpose |
|-----|---------|
| `POST /mcp` | JSON-RPC: `initialize`, `tools/list`, `tools/call` |
| `GET /mcp` | Returns 405 (use POST) |
| `GET /.well-known/mcp.json` | Discovery manifest for clients |

**Transport note:** The server implements **JSON-RPC over `POST /mcp`** as used by many MCP HTTP clients. It does **not** expose an **SSE** session or full **Streamable HTTP** session negotiation from the 2025 MCP transport spec—clients should use **stateless POST** to this endpoint. Unknown JSON-RPC methods return an **`error` object in the JSON body** with HTTP **200** (JSON-RPC style), not HTTP 404.

**Tools:** `search_agents` (`query`, optional `limit`), `get_agent_details` (`agent_id` UUID).

**Cursor:** use the “Add AgentRank search to your AI” section on `/connect` when `VITE_SEARCH_API_BASE_URL` is set, or install manually with MCP URL `{searchd}/mcp`.

## Structured A2A (no NL)

| URL | Purpose |
|-----|---------|
| `POST /v1/a2a` | JSON body: `skill` = `search_agents` \| `get_agent_details` |
| `GET /.well-known/agent-card.json` | Agent Card (alias: `/.well-known/agent.json`) |

**`search_agents`:** `{ "skill": "search_agents", "query": "...", "limit": 10 }`

**`get_agent_details`:** `{ "skill": "get_agent_details", "agent_id": "<uuid>" }`

Free-form **`text`** is rejected with `nl_not_supported` until a future NL milestone.

## REST

See [openapi/search-v0.1.yaml](../../apps/agentrank/openapi/search-v0.1.yaml).
