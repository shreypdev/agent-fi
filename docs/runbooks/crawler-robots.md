# Runbook: robots.txt + agent-robots.txt (Week 8)

## Fetch order

1. Site `robots.txt` at `{origin}/robots.txt` (cached).
2. **`/.well-known/agent-robots.txt`** at the same origin (separate cache key).

## Merge rule

- **`merged_robots_allow(site, agent, path)`** — path is allowed only if **both** `CachedRobots` evaluations allow it (deny if either ruleset denies).
- Missing file (404) is cached as **allow-all** until TTL.

## References

- Implementation: `crates/crawl-policy` fetch + `merged_robots_allow`.
- Ingest and probe paths use the same merge before outbound HEAD/GET.
