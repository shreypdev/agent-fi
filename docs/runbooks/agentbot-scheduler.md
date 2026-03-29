# AgentBot scheduled discover

## Env

| Variable | Meaning |
|----------|---------|
| `AGENTBOT_DISCOVER_INTERVAL_SECS` | Seconds between ticks (default `3600`). Set `0` to disable the background task. |
| `AGENTBOT_DISCOVER_HTTP_JSONS` | Comma-separated URLs returning `{"urls":[...]}` or a JSON array. |
| `PULSEMCP_FEED_URL` | Optional HTTP JSON feed (same format as `http_json_urls`). |
| `MCPSO_FEED_URL` | Optional second registry feed. |

## Behavior

On each tick, AgentBot enqueues URLs from: **builtin demo seed**, each **HTTP JSON** feed, **PulseMCP** and **mcp.so** feeds when env vars are set.

Failures log a warning and do not stop the run-loop.

## Railway

Run **one** long-lived `agentbot` service with `run-loop` + scheduler in-process (see `docker/entrypoint-agentbot.sh`). Alternative: Railway Cron to invoke `agentbot discover …` if you prefer isolation.
