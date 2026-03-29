# Runbook: domain probing (Week 8)

## Behavior

- **HEAD:** 5s timeout per candidate URL (`/.well-known/agent.json`, `agent-sitemap.xml`, `/agent.json`, `/api/agent-card`).
- **GET:** If HEAD is inconclusive, GET with **byte cap** (`AGENTBOT_PROBE_MAX_BODY_BYTES`, default 32 KiB) and JSON sniff (`{` or `[`).
- **Subdomains:** `agents.`, `agent.`, `api.`, `a2a.`, `mcp.` — **`tokio::net::lookup_host`** only; HTTP is skipped if resolution fails.
- **Seeds:** `AGENTBOT_PROBE_SEED_DOMAINS` (comma-separated). Enable **`AGENTBOT_PROBE_SCHEDULED=1`** to run the probe tick inside the discover scheduler.

## Dedup

- Redis key `probe:seen:{host}:{path}` with TTL **30 days** (`set_ex`).

## Metrics

- `agentrank_domain_probe_total{result=found|miss|error}`
- `agentrank_domain_probe_latency_seconds`
- `agentrank_probe_bytes_total`

## §B (HEAD→GET)

Integration tests should use Wiremock to assert HEAD-only success vs GET fallback; see `domain_probe` module tests (optional) and `a2a-discovery-todo.md` Week 8.
