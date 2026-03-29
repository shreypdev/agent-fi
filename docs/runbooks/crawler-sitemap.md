# Runbook: agent-sitemap discovery (Week 8)

## Behavior

- **Scheduled:** `AGENTBOT_SITEMAP_SCHEDULED=1` (default): each discover tick loads distinct `providers.primary_domain` rows, fetches `https://{domain}/.well-known/agent-sitemap.xml`, parses index or urlset (gzip supported), enqueues `<card_url>` / `<loc>` entries at high priority with `discovery_source=sitemap_scheduled` (or `sitemap_post_ingest` after ingest).
- **Post-ingest:** `AGENTBOT_SITEMAP_POST_INGEST=1` (default): after a successful card ingest, if Redis key `sitemap:{domain}` is **not** set, the same sitemap is fetched and siblings are enqueued; then the key is set with **24h TTL** to avoid duplicate work.

## Metrics

- `agentrank_sitemap_fetch_total{status}` — `ok`, `http_error`, `gzip_error`, `parse_error`, `error`.
- `agentrank_sitemap_cards_discovered_total{phase}` — `scheduled` / `post_ingest`.

## Compliance (Open-web crawler)

- Provenance: every enqueue carries `discovery_source` via frontier metadata (see §D in `a2a-discovery-todo.md`).
- Conditional fetch / `lastmod` + `sitemap_card_state` table can be extended for stricter If-Modified-Since; see migration `20260331120000_week8_crawler_v2.sql`.
