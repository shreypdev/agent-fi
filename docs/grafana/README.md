# Grafana dashboards (as code)

Import JSON into Grafana **Dashboards → Import** (or provision via your Grafana config).

| File | Purpose |
|------|---------|
| [`agentrank-overview.json`](./agentrank-overview.json) | HTTP rates (`searchd`), frontier depth, ingest outcomes, `frontier_enqueue_total`, `registry_discovered_urls_total`, `registry_connector_runs_total`. Set Prometheus datasource UID to match `${DS_PROMETHEUS}` or replace variable after import. |

## Frontier metrics

- **`frontier_enqueue_total{result,source}`** — `inserted` vs `updated` (dedupe / priority refresh).
- **`registry_discovered_urls_total{source}`** — URLs returned by a discover step before enqueue.
- **`registry_connector_runs_total{source,status}`** — successful discover CLI runs (`status=ok`); extend for errors when instrumented.
- **Dup rate:** use a ratio of `updated` to total enqueues over a window (see markdown panel in the dashboard JSON) — not a separate gauge.

## Suggested alert (one end-to-end)

In Grafana Alerting (or Alertmanager), add a rule on Prometheus:

- **Condition:** `absent(up{job="searchd"} == 1)` for 5m **or** probe `searchd` `GET /ready` != 200 from a synthetic check.
- **Notify:** Slack / PagerDuty / email webhook — document the webhook URL in your internal runbook (not committed).

Railway: scrape `searchd` `/metrics` with `Authorization: Bearer $METRICS_BEARER_TOKEN` when set. AgentBot metrics require `AGENTBOT_METRICS_BIND` and a reachable scrape target.

## Index freshness (Week 6+)

- **`agentrank_index_lag_seconds`** (histogram) — seconds between `agents.updated_at` and successful Qdrant + Tantivy upsert after ingest.
- **Alert:** if **P95 > 15 minutes** for 1h, page ops (see `a2a-discovery-todo.md` index freshness SLO).
- **`agentrank_index_upsert_failures_total`** — failed upserts (see `index_jobs` table for retries).
