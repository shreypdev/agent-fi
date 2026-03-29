# AgentRank — Rust workspace (data plane + ingest)

Week 1 **repo + data plane**: PostgreSQL/Redis helpers, `healthd`, **sqlx** migrations.

Week 2 **ingest path**: Agent Card parser (`agentrank-card`), Redis frontier (`agentrank-frontier`), **`agentbot`** CLI (fetch → parse → persist).

Week 4 **crawl policy**: `agentrank-crawl-policy` (`robots.txt` parse/cache, outbound URL validation). **`searchd` does not fetch user URLs**; only **`agentbot`** egresses. See [`docs/security-fetch-policy.md`](../../docs/security-fetch-policy.md).

**Week 6–7:** `QDRANT_URL` + `AGENTRANK_EMBEDDER` (default `hash`) for hybrid search; **`POST /v1/hints`** enqueues only; **`AGENTRANK_PUBLIC_URL`** for `/.well-known` MCP + Agent Card; MCP at **`POST /mcp`**, structured A2A at **`POST /v1/a2a`**. Details: [`docs/api/mcp-a2a-searchd.md`](../../docs/api/mcp-a2a-searchd.md).

## Toolchain

Uses [rust-toolchain.toml](./rust-toolchain.toml) (pinned stable). Install via [rustup](https://rustup.rs/).

```bash
cd apps/agentrank
cargo build --workspace
cargo test --workspace
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Local PostgreSQL and Redis

Start Docker services from repo root:

```bash
docker compose -f apps/dev/docker-compose.yml up -d
```

Copy root `.env.example` to `.env` and `source` it, or export:

```bash
export DATABASE_URL=postgresql://agentrank:agentrank@127.0.0.1:5433/agentrank
export REDIS_URL=redis://127.0.0.1:6379
```

Apply migrations before ingest or integration tests:

```bash
cd apps/agentrank
sqlx migrate run --source migrations
```

## Testing (what runs where)

| Area | Command | Required env / services |
|------|---------|-------------------------|
| Parser unit tests | `cargo test -p agentrank-card --lib` | None |
| Proptest (no panic + valid-card round-trip) | `cargo test -p agentrank-card --test proptest_invariants` | None |
| JSON Schema contract | `cargo test -p agentrank-card --test schema_contract` | None (loads [schemas/agent_card_core.schema.json](./schemas/agent_card_core.schema.json)) |
| Frontier | `cargo test -p agentrank-frontier` | **`REDIS_URL`** (tests no-op early if unset; CI sets it) |
| Data plane | `cargo test -p agentrank-data-plane` | Dead-port Redis test does not need a server; live check tests optional |
| AgentBot integration | `cargo test -p agentrank-agentbot --test ingest_integration` | **`DATABASE_URL`** + migrations; uses local HTTP servers (no external network except DNS test below) |
| Search index | `cargo test -p agentrank-search-index` | None (Tantivy temp dirs); golden + integration |
| Search API | `cargo test -p agentrank-searchd` | **`DATABASE_URL`** + **`REDIS_URL`** + migrations (`live_search`); optional **`QDRANT_URL`** for [`hybrid_qdrant_e2e`](./crates/searchd/tests/hybrid_qdrant_e2e.rs) (BM25 + Qdrant RRF + MCP on hybrid path) |
| Full workspace | `cargo test --workspace` | **`DATABASE_URL`** + **`REDIS_URL`** for full coverage; **GitHub Actions** also runs **Qdrant** (`QDRANT_URL=http://127.0.0.1:6334`) so hybrid E2E runs in CI |

**Embeddings:** `AGENTRANK_EMBEDDER` defaults to deterministic **hash** vectors (768-d, L2-normalized)—good for CI and plumbing. Swap to **BGE / ONNX** (or an HTTP sidecar) later for production semantic quality without changing the hybrid **shape** (same `EMBEDDING_DIM`, Qdrant cosine).

Integration tests cover: happy ingest, HTTP 404 / bad JSON / oversized body, **3-hop redirects**, **policy limit on redirects** (6 hops), **connection refused**, **HTTPS to plain-HTTP** (TLS failure class), **client timeout** vs slow server, **`.invalid` DNS failure**, **32 concurrent ingests** same `external_id` (single `agents` row, many `crawl_history` rows), re-ingest upsert.

**Frontier:** dedup, strict priority order, **10K** stress, **12 concurrent dequeuers** (no duplicate pops), tied scores, **Redis connection refused** on dead port, **invalid Redis URL** parse failure.

**Parser:** 59 focused unit tests + **proptest** (arbitrary bytes never panic; generated valid minimal cards parse) + **draft-07 schema** check on normalized output ([schemas/agent_card_core.schema.json](./schemas/agent_card_core.schema.json) documents the ingest contract; extend when A2A publishes a machine-readable schema you pin).

**Not in-repo (optional later):** `cargo fuzz` on `parse_agent_card_bytes`, multi-region chaos, slowloris-specific harness, formal verification of `ON CONFLICT` under extreme DB load.

## Migrations

Install CLI (once):

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

Apply:

```bash
cd apps/agentrank
sqlx migrate run --source migrations
```

Create database if needed:

```bash
sqlx database create
```

## `healthd`

Probes PostgreSQL (`SELECT 1`) and Redis (`PING`). If **`QDRANT_URL`** is set and non-empty, also checks Qdrant (gRPC). Exits **`0`** only if all configured checks succeed.

```bash
cd apps/agentrank
export DATABASE_URL=...
export REDIS_URL=...
# optional: export QDRANT_URL=http://127.0.0.1:6334
cargo run -p agentrank-healthd --bin healthd
```

**Railway:** same Docker image as searchd; set **`AGENTRANK_PROCESS=healthd`** plus `DATABASE_URL` / `REDIS_URL` (and optional `QDRANT_URL`). Use **Cron**, not an always-on web service — see [`RAILWAY.md`](./RAILWAY.md) §4.

Logs: set `RUST_LOG=debug` for more detail.

## `agentbot` (AgentBot v0.2)

Fetches a card URL with `AgentBot/1.0` user-agent (1 MiB body limit, up to 5 redirects), validates/normalizes JSON, upserts `providers` + `agents`, appends `crawl_history`, ensures `trust_records` (`indexed`). Before fetch: **HTTPS-only** URL policy (see [`docs/security-fetch-policy.md`](../../docs/security-fetch-policy.md)), **`robots.txt`** for the origin, **per-host Redis rate limit** (default 2 req/s bucket). Redirect target is re-validated.

Configurable timeout for callers that need it:

```rust
agentrank_agentbot::http_client_with_timeout(std::time::Duration::from_secs(30))?;
```

```bash
cd apps/agentrank
export DATABASE_URL=...
export REDIS_URL=...
sqlx migrate run --source migrations

# Ingest one URL
cargo run -p agentrank-agentbot --bin agentbot -- ingest 'https://example.com/.well-known/agent.json'

# Frontier (Redis sorted set `agentrank:frontier:v0`)
cargo run -p agentrank-agentbot --bin agentbot -- enqueue 'https://example.com/.well-known/agent.json' --priority 10.0
cargo run -p agentrank-agentbot --bin agentbot -- run-once
cargo run -p agentrank-agentbot --bin agentbot -- drain --max 50
# Long-running crawl (Unix; optional Prometheus on AGENTBOT_METRICS_BIND, e.g. 127.0.0.1:9093)
export AGENTBOT_METRICS_BIND=127.0.0.1:9093
cargo run -p agentrank-agentbot --bin agentbot -- run-loop

# Re-upsert every row into Tantivy + Qdrant (repair; needs SEARCH_INDEX_PATH and optional QDRANT_URL)
cargo run -p agentrank-agentbot --bin agentbot -- index-backfill
```

Env (optional): `AGENTBOT_HOST_MAX_PER_SEC` (default `2`), `AGENTBOT_ROBOTS_TTL_OK_SECS`, `AGENTBOT_ROBOTS_TTL_NEGATIVE_SECS`, `AGENTBOT_METRICS_BIND`, `AGENTBOT_ALLOW_HTTP_LOCALHOST`, `AGENTBOT_ALLOW_LOOPBACK_HTTPS` (tests only).

## Week 3 — Search index + `searchd` API

**Index (`agentrank-search-index`):** Tantivy over `agents.name`, `agents.description`, and a deterministic **skills blob** from `card_json.skills` (per-skill: `name`, `tags`, `description`, `examples` — see `crates/search-index/src/extract.rs`). Max ~50k Unicode scalars per text field (stable truncation). Field boosts (frozen): name > skills > description (`crates/search-index/src/schema.rs`).

**Lifecycle:**

```bash
export SEARCH_INDEX_PATH=/path/to/index/dir   # must be writable
cargo run -p agentrank-search-index --bin agentrank-index -- rebuild --output "$SEARCH_INDEX_PATH"
cargo run -p agentrank-search-index --bin agentrank-index -- upsert --index "$SEARCH_INDEX_PATH" --agent-id '<uuid>'
```

**HTTP API (`searchd`):** `GET /health` (liveness), **`GET /ready`** (readiness: Postgres + Redis + index), `GET /metrics` (Prometheus), `POST /v1/search`, `GET /v1/agents/:id`. Contract: [`openapi/search-v0.1.yaml`](./openapi/search-v0.1.yaml).

```bash
export DATABASE_URL=...
export REDIS_URL=...
export SEARCH_INDEX_PATH=...
export PORT=8080   # Railway sets this automatically
cargo run -p agentrank-searchd --bin searchd
```

**Rate limit:** Redis fixed window 60s per client IP. **Railway:** `X-Forwarded-For` / `X-Real-IP` are trusted by default (`RAILWAY_*` env present). **`TRUST_PROXY_HEADERS=0`** forces TCP peer only; **`=1`** forces header trust off-Railway. Cap: `SEARCH_RATE_LIMIT_PER_MINUTE` (default `120`). Returns `429` + `Retry-After: 60`.

**CORS:** `CORS_ORIGINS` comma-separated list; if unset, any origin is allowed (demo-friendly). Set **`CORS_REQUIRE_ORIGINS=1`** in production so startup fails without explicit origins.

**Index CLI:** `agentrank-index probe --output "$SEARCH_INDEX_PATH"` verifies the on-disk index (used by Docker entrypoint when `SEARCHD_INDEX_BOOT=reuse`). See [`RAILWAY.md`](./RAILWAY.md) for volumes and multi-replica notes.

**Demo seed:** Migration `20260328200000_demo_seed_agents.sql` inserts two idempotent catalog rows (`ON CONFLICT (external_id) DO NOTHING`), including a card pointing at the public demo agent URL.

**Load / perf (local):**

- Generate bulk SQL: `python3 scripts/gen_1k_agents_sql.py --count 1000 > /tmp/seed.sql` then `psql "$DATABASE_URL" -f /tmp/seed.sql`, rebuild index.
- Latency smoke: [`../../scripts/search_p99.sh`](../../scripts/search_p99.sh) (expects `SEARCHD_URL`, default `http://127.0.0.1:8080`). Target: **P99 &lt; 200ms** on reference hardware with a warm index (not enforced in CI).

**Docker:** [`Dockerfile`](./Dockerfile) — one image for **searchd**, **consoled**, and **agentbot** (`AGENTRANK_PROCESS`). Example: `docker build -f apps/agentrank/Dockerfile -t agentrank .` from repo root. Railway: see [`RAILWAY.md`](./RAILWAY.md). Compose adds `searchd` in [`../dev/docker-compose.yml`](../dev/docker-compose.yml); **build the index into the volume first** (see dev README).

**Deferred (not Week 3):** hybrid vectors / Qdrant, rich `POST /v1/search` filters from opus §16, explain payloads, AVERT. Grafana/SLO: see `docs/grafana/` and [`RAILWAY.md`](./RAILWAY.md). **`agentbot`** outbound URL policy is implemented in `agentrank-crawl-policy`; DNS-rebinding-class hardening remains future work.

## Crates

| Crate                 | Role                                                |
| --------------------- | --------------------------------------------------- |
| `agentrank-data-plane` | `DATABASE_URL` / `REDIS_URL`, pools, health checks |
| `agentrank-healthd`    | Binary: operational smoke test                     |
| `agentrank-card`       | Agent Card parse, validate, normalize (`ParsedAgentCard`) |
| `agentrank-frontier`   | Redis ZSET URL queue: enqueue / dequeue / dedup     |
| `agentrank-crawl-policy` | `robots.txt` parse/cache + outbound URL validation |
| `agentrank-agentbot`   | Library + `agentbot` binary: HTTP ingest + DB writes |
| `agentrank-search-index` | Tantivy schema, indexer CLI `agentrank-index`     |
| `agentrank-searchd`    | Axum `searchd` binary                              |

## Schema v1

Tables: `providers`, `agents`, `crawl_history`, `trust_records`. See `migrations/20260328120000_initial_schema.sql`.

Default `trust_records.trust_tier` is `indexed`. Documented progression (app-enforced): `indexed` → `established` → `verified` → `trusted` → `authoritative`.
