# Registry connector: `builtin_demo_seed`

- **Source id:** `builtin_demo_seed`
- **Command:** `cargo run -p agentrank-agentbot --bin agentbot -- discover builtin`
- **Behavior:** Enqueues a small static list of public card URLs (code in `agentrank-registry-connectors`).
- **Ops:** Safe offline smoke; expand the list in code when you add vetted URLs.
