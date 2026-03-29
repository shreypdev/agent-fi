# GitHub discovery (`discover github`)

- **Command:** `GITHUB_TOKEN=ghp_... cargo run -p agentrank-agentbot --bin agentbot -- discover github 'filename:agent.json path:.well-known' --max 10 --priority 5`
- **API:** GitHub REST code search. Requires a **classic PAT** or fine-grained token with **code search** access where applicable.
- **Test / mock:** set **`GITHUB_API_BASE_URL`** to a wiremock root (e.g. `http://127.0.0.1:PORT`) plus a fake **`GITHUB_TOKEN`**; production leaves `GITHUB_API_BASE_URL` unset.
- **Rate limits:** Watch response headers `X-RateLimit-Remaining`. On `403` / rate limit, pause jobs or raise token tier.
- **Output URLs:** Best-effort `raw.githubusercontent.com/{owner}/{repo}/{main|master}/.well-known/agent.json` (duplicates possible; agentbot dedupes frontier by URL).
- **CI:** Do not put `GITHUB_TOKEN` in PR workflows; run manually or from a protected environment.
