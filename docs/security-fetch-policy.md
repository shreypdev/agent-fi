# Outbound fetch policy (AgentBot)

AgentRank splits responsibilities:

- **`searchd`** serves `POST /v1/search` and **never** performs user-driven HTTP fetches. Callers supply queries only; there is no SSRF surface on the search API for arbitrary URLs. CI fails if `reqwest` / `ureq` appear under `crates/searchd/src` (see `.github/workflows/ci.yml`).
- **`POST /v1/hints`** only **validates** the submitted URL (`validate_outbound_url`) and enqueues it on the Redis frontier — **no HTTP GET** from `searchd`. The crawl worker (`agentbot`) performs the actual fetch under the same policy as other ingests.
- **`agentbot`** is the **only** production component that fetches third-party URLs (Agent Cards, `robots.txt`). All SSRF mitigations apply there.

## Rules enforced (`agentrank-crawl-policy`)

Implementation: [`apps/agentrank/crates/crawl-policy/src/url_policy.rs`](../apps/agentrank/crates/crawl-policy/src/url_policy.rs).

| Check | Behavior |
|-------|----------|
| Scheme | Default: **HTTPS only**. Optional `http://127.0.0.1` / `localhost` / `[::1]` when `AGENTBOT_ALLOW_HTTP_LOCALHOST=1` (tests, local wiremock). |
| Loopback HTTPS | Optional `https://127.0.0.1` when `AGENTBOT_ALLOW_LOOPBACK_HTTPS=1` (integration tests). **Never enable in production** against untrusted input. |
| Userinfo | URLs with `user:pass@host` are rejected. |
| Literal IPs | Private, loopback (unless exceptions above), link-local, shared-space (100.64.0.0/10), documentation, unspecified, and **169.254.169.254** are blocked for normal HTTPS fetches. |
| Hostnames | `metadata.google.internal`, host `metadata`, and `*.internal` suffix are blocked (cloud metadata class). |
| Redirects | After `GET`, the **final** URL is validated the same way; redirects to private IPs are rejected (`PostRedirectPolicy` in ingest). |

## Residual risk: DNS rebinding

Validation runs on the **URL string** and the resolved connection target may differ in edge cases (DNS rebinding, race conditions). Mitigation in depth: short timeouts, limited redirects, and operating AgentBot in a network posture that does not expose internal services. Document any stricter controls (egress firewall, resolved-IP check before connect) as future hardening.

## Environment variables

| Variable | Purpose |
|----------|---------|
| `AGENTBOT_ALLOW_HTTP_LOCALHOST` | `1` / `true`: allow `http://127.0.0.1` and `localhost` fetches. |
| `AGENTBOT_ALLOW_LOOPBACK_HTTPS` | `1` / `true`: allow `https://127.0.0.1` (tests only). |

Production: leave both **unset**.

## Related

- [`apps/agentrank/README.md`](../apps/agentrank/README.md) — AgentBot CLI and crawl loop.
- [`a2a-discovery-opus.md`](../a2a-discovery-opus.md) — product crawl / robots policy (incl. future `agent-robots.txt`).
