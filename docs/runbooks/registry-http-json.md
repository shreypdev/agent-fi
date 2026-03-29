# Registry connector: `http_json_urls`

- **Source id:** `http_json_urls`
- **Command:** `cargo run -p agentrank-agentbot --bin agentbot -- discover http-json 'https://feed.example/registry.json' --priority 0`
- **Feed format:** JSON object `{ "urls": [ "https://...", ... ] }` or a top-level string array.
- **Failure modes:** Non-2xx → error logged; malformed JSON → `InvalidFeed`.
- **429 / 5xx:** Retry with backoff at orchestration layer (not implemented in connector); reduce `priority` to avoid starving the frontier.
