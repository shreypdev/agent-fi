# Registry connector: `static_json_file`

- **Source id:** `static_json_file`
- **Command:** `cargo run -p agentrank-agentbot --bin agentbot -- discover file ./path/to/urls.json --priority 2`
- **File format:** Same as HTTP JSON feed: `{ "urls": [ "https://...", ... ] }` or a top-level string array.
- **Use cases:** Air-gapped CI, curated seed lists, reproducible demos without hitting a remote feed.
