# AgentRank — Rust workspace (data plane + ingest)

Week 1 **repo + data plane**: PostgreSQL/Redis helpers, `healthd`, **sqlx** migrations.

Week 2 **ingest path**: Agent Card parser (`agentrank-card`), Redis frontier (`agentrank-frontier`), **`agentbot`** CLI (fetch → parse → persist).

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
| Full workspace | `cargo test --workspace` | **`DATABASE_URL`** + **`REDIS_URL`** for full coverage |

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

Probes PostgreSQL (`SELECT 1`) and Redis (`PING`). Exits `0` only if both succeed.

```bash
cd apps/agentrank
export DATABASE_URL=...
export REDIS_URL=...
cargo run -p agentrank-healthd --bin healthd
```

Logs: set `RUST_LOG=debug` for more detail.

## `agentbot` (AgentBot v0.1)

Fetches a card URL with `AgentBot/1.0` user-agent (1 MiB body limit, up to 5 redirects), validates/normalizes JSON, upserts `providers` + `agents`, appends `crawl_history`, ensures `trust_records` (`indexed`).

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
```

## Crates

| Crate                 | Role                                                |
| --------------------- | --------------------------------------------------- |
| `agentrank-data-plane` | `DATABASE_URL` / `REDIS_URL`, pools, health checks |
| `agentrank-healthd`    | Binary: operational smoke test                     |
| `agentrank-card`       | Agent Card parse, validate, normalize (`ParsedAgentCard`) |
| `agentrank-frontier`   | Redis ZSET URL queue: enqueue / dequeue / dedup     |
| `agentrank-agentbot`   | Library + `agentbot` binary: HTTP ingest + DB writes |

## Schema v1

Tables: `providers`, `agents`, `crawl_history`, `trust_records`. See `migrations/20260328120000_initial_schema.sql`.

Default `trust_records.trust_tier` is `indexed`. Documented progression (app-enforced): `indexed` → `established` → `verified` → `trusted` → `authoritative`.
