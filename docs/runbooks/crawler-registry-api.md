# Runbook: paginated registry APIs (Week 8)

## Connectors

| Env | Connector |
|-----|-----------|
| `PULSEMCP_API_BASE`, `PULSEMCP_TENANT_ID`, `PULSEMCP_API_KEY` | PulseMCP (`/api/v0.1/servers`) |
| `AGENTVERSE_API_BASE`, `AGENTVERSE_API_KEY` | AgentVerse REST |
| `MCPSO_REGISTRY_JSON_URL` | mcp.so-style JSON (`entries` or top-level array) |
| `GENERIC_REGISTRY_*` | Config-driven JSON (see `.env.example`) |

## State

- Table **`registry_sync_state`**: `last_cursor`, `last_sync_at`, `total_synced`, `errors`.
- Requires **`DATABASE_URL`** and **`AGENTBOT_REGISTRY_PAGINATION=1`** (default) on the discover scheduler.

## Staging

- Operator goal: **≥5** real feeds with API keys (P0-02). Validate in logs: `registry pagination` and frontier `discovery_source` labels.
